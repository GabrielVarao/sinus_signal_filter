# Rust Signal Filter Lab

Ein kleines Signalverarbeitungs-Projekt mit **Rust**, **Python**, **ZMQ** und **Protobuf**.

Rust erzeugt live ein künstliches Messsignal, fügt ca. **5% Rauschen** hinzu und glättet das Signal mit einem **Moving-Average-Filter**.  
Die Daten werden mit **Protobuf** serialisiert und über **ZMQ** in Echtzeit an Python gesendet. Python empfängt die Daten und zeigt sie live als Graph an.

---

## Funktionen

- zufälliges Messsignal generieren
- ca. 5% Rauschen hinzufügen
- Signal mit Moving-Average-Filter glätten
- Daten mit Protobuf codieren
- Daten live über ZMQ senden
- Live-Plot mit Python anzeigen
- Originalsignal, verrauschtes Signal und gefiltertes Signal vergleichen

---

## Demo

[Demo-Video ansehen](docs/Aufzeichnung%202026-05-29%20121039.mp4)

---

## Projektstruktur

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