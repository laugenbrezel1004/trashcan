import toml

with open("Cargo.toml", "r") as file:
    data = toml.load(file)
    version = data.get("package", {}).get("version")
    print(version)
    (x, y, z) = map(int, version.split("."))
    z+=1
    data.setdefault("package", {}).setdefault("version", str(f"{x}.{y}.{z}"))