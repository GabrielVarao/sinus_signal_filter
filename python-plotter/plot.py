import pandas as pd
import matplotlib.pyplot as plt
from pathlib import Path


def choose_csv_file(data_folder: Path) -> Path | None:
    csv_files = sorted(data_folder.glob("signal_data_*.csv"))

    if not csv_files:
        print("Keine CSV-Dateien gefunden.")
        print("Starte zuerst das Rust-Programm mit: cargo run")
        return None

    print("\nVerfügbare CSV-Dateien:\n")

    for index, file in enumerate(csv_files, start=1):
        print(f"{index}: {file.name}")

    while True:
        choice = input("\nWelche Datei möchtest du visualisieren? Nummer eingeben: ")

        if not choice.isdigit():
            print("Bitte eine Zahl eingeben.")
            continue

        choice_index = int(choice) - 1

        if 0 <= choice_index < len(csv_files):
            return csv_files[choice_index]

        print("Ungültige Nummer.")


def plot_signal(csv_path: Path):
    data = pd.read_csv(csv_path)

    required_columns = {"time", "clean", "noisy", "filtered"}

    if not required_columns.issubset(data.columns):
        print("CSV-Datei hat nicht die erwarteten Spalten.")
        print("Erwartet:", required_columns)
        print("Gefunden:", set(data.columns))
        return

    plt.figure(figsize=(12, 6))

    plt.plot(data["time"], data["clean"], label="Originalsignal")
    plt.plot(data["time"], data["noisy"], label="Verrauschtes Signal")
    plt.plot(data["time"], data["filtered"], label="Gefiltertes Signal")

    plt.title(f"Rust Signal Filter Lab\n{csv_path.name}")
    plt.xlabel("Zeit / Sample")
    plt.ylabel("Signalwert")
    plt.legend()
    plt.grid(True)

    plt.tight_layout()
    plt.savefig("signal_plot.png", dpi=200)
    plt.show()


def main():
    data_folder = Path("../data")
    selected_file = choose_csv_file(data_folder)

    if selected_file is None:
        return

    print(f"\nVisualisiere: {selected_file.name}")
    plot_signal(selected_file)


if __name__ == "__main__":
    main()