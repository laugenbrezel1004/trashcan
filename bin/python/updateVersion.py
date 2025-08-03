import os
import toml

os.chdir(os.path.dirname(os.path.abspath(__file__)))
def increment_version(version):
    try:
        # Versionsnummer in x, y, z aufteilen
        x, y, z = map(int, version.split('.'))

        # z um 1 erhöhen
        z += 1

        # Übertrag handhaben
        if z > 9:  # z geht nur bis 9
            z = 0
            y += 1
        if y > 9:  # Annahme: y hat eine Obergrenze von 99
            y = 0
            x += 1

        # Neue Versionsnummer formatieren
        return f"{x}.{y}.{z}"
    except ValueError:
        return "Ungültiges Versionsformat. Erwartet: x.y.z"




# Datei öffnen und bearbeiten
try:
    with open("../../Cargo.toml", "r+") as file:
        # TOML-Datei laden
        data = toml.load(file)

        # Versionsnummer abrufen
        version = data.get("package", {}).get("version")
        if not version:
            print("Keine Versionsnummer gefunden in [package].")
            exit(1)

        print("Alte Version:", version)

        # Neue Versionsnummer berechnen
        new_version = increment_version(version)
        print("Neue Version:", new_version)

        # Versionsnummer im Dictionary aktualisieren
        if "package" in data:
            data["package"]["version"] = new_version
        else:
            data["package"] = {"version": new_version}

        # Dateizeiger an den Anfang setzen
        file.seek(0)

        # Geänderte Daten zurück in die Datei schreiben
        toml.dump(data, file)

        # Datei kürzen, falls der neue Inhalt kürzer ist
        file.truncate()

except FileNotFoundError:
    print("Datei '../../Cargo.toml' nicht gefunden!")
except toml.TomlDecodeError:
    print("Fehler beim Parsen der TOML-Datei!")
except Exception as e:
    print(f"Ein Fehler ist aufgetreten: {e}")
