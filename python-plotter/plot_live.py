from collections import deque

import matplotlib.pyplot as plt
import zmq

from signal_pb2 import SignalSample


ZMQ_ADDRESS = "tcp://127.0.0.1:5555"
MAX_POINTS = 300


def main():
    print("Python Live Plotter")
    print(f"Connecting to ZMQ publisher on {ZMQ_ADDRESS}")
    print("Waiting for protobuf signal samples...")

    # Create ZMQ subscriber socket.
    context = zmq.Context()
    socket = context.socket(zmq.SUB)
    socket.connect(ZMQ_ADDRESS)

    # Subscribe to all messages.
    socket.setsockopt_string(zmq.SUBSCRIBE, "")

    # Store latest values.
    times = deque(maxlen=MAX_POINTS)
    clean_values = deque(maxlen=MAX_POINTS)
    noisy_values = deque(maxlen=MAX_POINTS)
    filtered_values = deque(maxlen=MAX_POINTS)

    # Enable interactive plotting.
    plt.ion()
    fig, ax = plt.subplots(figsize=(12, 6))

    clean_line, = ax.plot([], [], label="Clean signal")
    noisy_line, = ax.plot([], [], label="Noisy signal")
    filtered_line, = ax.plot([], [], label="Filtered signal")

    ax.set_title("Live Rust Signal Filter Lab")
    ax.set_xlabel("Time / Sample")
    ax.set_ylabel("Signal value")
    ax.set_ylim(0, 100)
    ax.grid(True)
    ax.legend()

    try:
        while True:
            try:
                # Receive protobuf message without blocking forever.
                raw_message = socket.recv(flags=zmq.NOBLOCK)
            except zmq.Again:
                plt.pause(0.01)
                continue

            sample = SignalSample()
            sample.ParseFromString(raw_message)

            times.append(sample.time)
            clean_values.append(sample.clean)
            noisy_values.append(sample.noisy)
            filtered_values.append(sample.filtered)

            clean_line.set_data(times, clean_values)
            noisy_line.set_data(times, noisy_values)
            filtered_line.set_data(times, filtered_values)

            if len(times) > 1:
                ax.set_xlim(min(times), max(times))

            fig.canvas.draw()
            fig.canvas.flush_events()

            print(
                f"Received sample {sample.time:>3}: "
                f"clean={sample.clean:>6.2f}, "
                f"noisy={sample.noisy:>6.2f}, "
                f"filtered={sample.filtered:>6.2f}"
            )

            plt.pause(0.001)

    except KeyboardInterrupt:
        print("\nStopping live plotter...")

    finally:
        socket.close()
        context.term()


if __name__ == "__main__":
    main()