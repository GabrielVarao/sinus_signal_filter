use prost::Message;
use rand::Rng;
use std::collections::VecDeque;
use std::error::Error;
use std::time::Duration;
use zeromq::{PubSocket, Socket, SocketSend};

// This module is generated automatically from proto/signal.proto.
pub mod signal {
    include!(concat!(env!("OUT_DIR"), "/signal.rs"));
}

const SAMPLE_COUNT: usize = 600;
const MIN_VALUE: f64 = 0.0;
const MAX_VALUE: f64 = 100.0;
const NOISE_PERCENT: f64 = 0.05;
const FILTER_WINDOW: usize = 7;
const ZMQ_ADDRESS: &str = "tcp://127.0.0.1:5555";

struct SignalProfile {
    start_value: f64,
    ramp_target: f64,
    peak_value: f64,
    end_value: f64,
    stable_until: usize,
    ramp_until: usize,
    plateau_until: usize,
    peak_until: usize,
    fall_until: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Rust Signal Filter Lab");
    println!("Starting ZMQ publisher on {}", ZMQ_ADDRESS);
    println!("Sending protobuf encoded signal samples...");

    // Create a ZeroMQ PUB socket.
    let mut publisher = PubSocket::new();
    publisher.bind(ZMQ_ADDRESS).await?;

    // Give the Python subscriber a short moment to connect.
    tokio::time::sleep(Duration::from_millis(1000)).await;

    let mut rng = rand::thread_rng();
    let profile = generate_random_profile(&mut rng);
    let mut filter_buffer: VecDeque<f64> = VecDeque::new();

    println!("Random signal profile:");
    println!("Start value: {:.1}", profile.start_value);
    println!("Ramp target: {:.1}", profile.ramp_target);
    println!("Peak value:  {:.1}", profile.peak_value);
    println!("End value:   {:.1}", profile.end_value);

    for time in 0..SAMPLE_COUNT {
        // Generate clean signal value.
        let clean = generate_clean_signal(time, &profile);

        // Add random noise.
        let noise = generate_noise(&mut rng);
        let noisy = clamp(clean + noise, MIN_VALUE, MAX_VALUE);

        // Filter noisy signal.
        let filtered = moving_average_filter(&mut filter_buffer, noisy);

        // Create protobuf message.
        let sample = signal::SignalSample {
            time: time as u32,
            clean,
            noisy,
            filtered,
        };

        // Encode protobuf message into binary data.
        let mut buffer = Vec::new();
        sample.encode(&mut buffer)?;

        // Send binary protobuf data over ZeroMQ.
        publisher.send(buffer.into()).await?;

        println!(
            "Sent sample {:>3}: clean={:>6.2}, noisy={:>6.2}, filtered={:>6.2}",
            time, clean, noisy, filtered
        );

        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    println!("Finished sending live signal data.");
    Ok(())
}

// Creates a random signal profile for each program run.
fn generate_random_profile(rng: &mut rand::rngs::ThreadRng) -> SignalProfile {
    let start_value = rng.gen_range(20.0..40.0);
    let ramp_target = rng.gen_range(50.0..70.0);
    let peak_value = rng.gen_range(72.0..95.0);
    let end_value = rng.gen_range(35.0..60.0);

    let stable_until = rng.gen_range(35..70);
    let ramp_until = rng.gen_range(110..150);
    let plateau_until = rng.gen_range(170..210);
    let peak_until = rng.gen_range(215..245);
    let fall_until = rng.gen_range(260..290);

    SignalProfile {
        start_value,
        ramp_target,
        peak_value,
        end_value,
        stable_until,
        ramp_until,
        plateau_until,
        peak_until,
        fall_until,
    }
}

// Generates a clean artificial sensor signal.
fn generate_clean_signal(time: usize, profile: &SignalProfile) -> f64 {
    if time < profile.stable_until {
        profile.start_value
    } else if time < profile.ramp_until {
        interpolate(
            time,
            profile.stable_until,
            profile.ramp_until,
            profile.start_value,
            profile.ramp_target,
        )
    } else if time < profile.plateau_until {
        profile.ramp_target
    } else if time < profile.peak_until {
        profile.peak_value
    } else if time < profile.fall_until {
        interpolate(
            time,
            profile.peak_until,
            profile.fall_until,
            profile.peak_value,
            profile.end_value,
        )
    } else {
        profile.end_value
    }
}

// Linear interpolation between two values.
fn interpolate(
    time: usize,
    start_time: usize,
    end_time: usize,
    start_value: f64,
    end_value: f64,
) -> f64 {
    let progress = (time - start_time) as f64 / (end_time - start_time) as f64;
    start_value + progress * (end_value - start_value)
}

// Generates random noise of approximately +/- 5% of the measurement range.
fn generate_noise(rng: &mut rand::rngs::ThreadRng) -> f64 {
    let noise_range = (MAX_VALUE - MIN_VALUE) * NOISE_PERCENT;
    rng.gen_range(-noise_range..=noise_range)
}

// Moving average filter.
// It averages the latest values to smooth the noisy signal.
fn moving_average_filter(buffer: &mut VecDeque<f64>, new_value: f64) -> f64 {
    buffer.push_back(new_value);

    if buffer.len() > FILTER_WINDOW {
        buffer.pop_front();
    }

    let sum: f64 = buffer.iter().sum();
    sum / buffer.len() as f64
}

// Limits values to the valid measurement range.
fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}