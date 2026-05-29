use chrono::Local;
use rand::Rng;
use serde::Serialize;
use std::collections::VecDeque;
use std::error::Error;

const SAMPLE_COUNT: usize = 300;
const MIN_VALUE: f64 = 0.0;
const MAX_VALUE: f64 = 100.0;
const NOISE_PERCENT: f64 = 0.05;
const FILTER_WINDOW: usize = 7;

#[derive(Serialize)]
struct SignalSample {
    time: usize,
    clean: f64,
    noisy: f64,
    filtered: f64,
}

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

fn main() -> Result<(), Box<dyn Error>> {
    println!("Rust Signal Filter Lab");
    println!("Generating random signal data...");

    let mut rng = rand::thread_rng();
    let mut filter_buffer: VecDeque<f64> = VecDeque::new();

    let profile = generate_random_profile(&mut rng);

    println!("Random signal profile:");
    println!("Start value: {:.1}", profile.start_value);
    println!("Ramp target: {:.1}", profile.ramp_target);
    println!("Peak value:  {:.1}", profile.peak_value);
    println!("End value:   {:.1}", profile.end_value);

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let output_path = format!("../data/signal_data_{}.csv", timestamp);

    let mut writer = csv::Writer::from_path(&output_path)?;

    for time in 0..SAMPLE_COUNT {
        let clean = generate_clean_signal(time, &profile);
        let noise = generate_noise(&mut rng);
        let noisy = clamp(clean + noise, MIN_VALUE, MAX_VALUE);
        let filtered = moving_average_filter(&mut filter_buffer, noisy);

        let sample = SignalSample {
            time,
            clean,
            noisy,
            filtered,
        };

        writer.serialize(sample)?;
    }

    writer.flush()?;

    println!("Done!");
    println!("CSV saved to: {}", output_path);

    Ok(())
}

// Erstellt für jeden Programmstart ein neues zufälliges Signalprofil.
// Dadurch sieht jede CSV-Datei wirklich anders aus.
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

// Erzeugt ein künstliches Messsignal anhand eines zufälligen Profils.
// Das Signal ist jetzt bei jedem Programmstart anders.
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

// Lineare Interpolation.
// Damit kann das Signal sauber von einem Wert zu einem anderen Wert steigen/fallen.
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

// Erzeugt zufälliges Rauschen von ungefähr ±5%.
// Bei Messbereich 0 bis 100 bedeutet 5% ungefähr ±5.
fn generate_noise(rng: &mut rand::rngs::ThreadRng) -> f64 {
    let noise_range = (MAX_VALUE - MIN_VALUE) * NOISE_PERCENT;
    rng.gen_range(-noise_range..=noise_range)
}

// Moving-Average-Filter.
// Er nimmt die letzten Werte und bildet daraus den Durchschnitt.
// Dadurch wird das verrauschte Signal geglättet.
fn moving_average_filter(buffer: &mut VecDeque<f64>, new_value: f64) -> f64 {
    buffer.push_back(new_value);

    if buffer.len() > FILTER_WINDOW {
        buffer.pop_front();
    }

    let sum: f64 = buffer.iter().sum();
    sum / buffer.len() as f64
}

// Begrenzt Werte auf den erlaubten Messbereich.
fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}