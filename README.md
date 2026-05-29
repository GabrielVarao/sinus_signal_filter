# Rust Signal Filter Lab

Ein Signalverarbeitungs-Projekt mit **Rust**, **Python**, **ZMQ** und **Protobuf**.

Rust erzeugt live ein künstliches Messsignal, fügt ca. **5% Rauschen** hinzu und glättet das Signal mit einem **Moving-Average-Filter**.
Die Daten werden mit **Protobuf** serialisiert und über **ZMQ** in Echtzeit an Python gesendet. Python empfängt die Daten und zeigt sie live als Graph an.

---

## Features

* Zufälliges Messsignal generieren
* Ca. 5% Rauschen hinzufügen
* Signal mit Moving-Average-Filter glätten
* Gemeinsames Datenformat mit Protobuf
* Live-Datenübertragung von Rust zu Python mit ZMQ
* Echtzeit-Visualisierung mit Python und Matplotlib
* Vergleich von Originalsignal, verrauschtem Signal und gefiltertem Signal

---

## Demo

[Demo-Video ansehen](docs/Aufzeichnung%202026-05-29%20121039.mp4)

---

## Project Structure

```text
rust-signal-filter-lab
├── proto
│   └── signal.proto
├── rust-signal-generator
│   ├── Cargo.toml
│   ├── build.rs
│   └── src
│       └── main.rs
├── python-plotter
│   ├── plot_live.py
│   ├── signal_pb2.py
│   └── requirements.txt
├── data
│   └── .gitkeep
├── docs
│   └── Aufzeichnung 2026-05-29 121039.mp4
└── README.md
```

---

## Technologies

| Technology | Usage                                  |
| ---------- | -------------------------------------- |
| Rust       | Generates and filters the signal data  |
| Python     | Visualizes the live signal data        |
| ZMQ        | Sends live data from Rust to Python    |
| Protobuf   | Defines and serializes the data format |
| Matplotlib | Draws the live graph                   |

---

## Data Format

The shared Protobuf schema is defined in:

```text
proto/signal.proto
```

```proto
syntax = "proto3";

package signal;

message SignalSample {
  uint32 time = 1;
  double clean = 2;
  double noisy = 3;
  double filtered = 4;
}
```

Each message contains one signal sample:

| Field      | Description                     |
| ---------- | ------------------------------- |
| `time`     | Sample index                    |
| `clean`    | Original clean signal           |
| `noisy`    | Signal with around 5% noise     |
| `filtered` | Smoothed signal after filtering |

---

## Requirements

You need:

* Rust and Cargo
* Python
* pip

Check installations:

```bash
cargo --version
python --version
pip --version
```

---

## Setup Python Environment

Go into the Python folder:

```bash
cd python-plotter
```

Create a virtual environment:

```bash
python -m venv .venv
```

Activate the virtual environment on Windows:

```bash
.venv\Scripts\activate
```

Install the required packages:

```bash
pip install -r requirements.txt
```

If `signal_pb2.py` does not exist yet, generate the Python Protobuf file:

```bash
python -m grpc_tools.protoc -I ../proto --python_out . ../proto/signal.proto
```

---

## Run the Live Application

Important: **Start Python first, then Rust.**

### Terminal 1: Start Python Live Plotter

```bash
cd python-plotter
.venv\Scripts\activate
python plot_live.py
```

Python now waits for live signal data.

### Terminal 2: Start Rust Signal Generator

```bash
cd rust-signal-generator
cargo run
```

Rust now sends live signal data to Python.
Python receives the data and updates the graph in real time.

---

## How It Works

Rust generates an artificial sensor signal.
The signal is slightly different on every run because a random signal profile is created.

Then random noise is added:

```text
noisy = clean + random noise
```

The noise is around 5% of the measurement range.

After that, the noisy signal is smoothed with a Moving-Average-Filter:

```text
filtered = average of the latest values
```

Each sample is encoded with Protobuf and sent over ZMQ to Python.

Python receives the binary data, decodes it with Protobuf and updates the graph live.

---

## Moving-Average-Filter

The project uses a **Moving-Average-Filter**.

This filter calculates the average of the latest values.
Random spikes are reduced and the signal becomes smoother.

Example:

```text
Values: 52, 48, 51, 53, 50
Average: 50.8
```

The filtered signal is smoother than the noisy signal.

---

## Why ZMQ and Protobuf?

**ZMQ** is used so that Rust and Python can communicate in real time.
Rust sends live measurement data and Python receives it directly.

**Protobuf** is used so both programs share the same structured data format.
This makes the transmitted data compact and clearly defined.

This combination is useful for:

* Live measurement systems
* Sensor simulations
* Monitoring tools
* Prototyping
* Distributed applications

---

## Unit Tests and Test Driven Design

Rust has built-in support for unit tests with Cargo.

Tests can be started with:

```bash
cargo test
```

Useful test cases for this project would be:

* `clamp()` limits values correctly
* `interpolate()` calculates values correctly
* `moving_average_filter()` smooths values correctly
* generated signal values stay inside the valid range

Test Driven Design means writing tests before writing or changing the actual function.
For this project, this would be especially useful for the signal and filter functions.

---

## Possible Extensions

Possible next steps:

* Add more filter types
* Add CSV export again next to live transmission
* Make noise strength configurable
* Make signal type selectable
* Add a small Python GUI
* Save live data for later analysis
* Compare different filters in the graph
* Add automated Rust unit tests
* Add Python tests with pytest

---

## Short Summary

This project simulates a noisy measurement signal.
Rust generates and filters the data. The samples are encoded with Protobuf and sent live over ZMQ to Python.
Python visualizes the original signal, the noisy signal and the filtered signal in real time.
