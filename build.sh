#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"

cargo build --release --locked --target wasm32-unknown-unknown
package=dist/solidinserts
mkdir -p "$package"
wasm-tools component new "${CARGO_TARGET_DIR:-target}/wasm32-unknown-unknown/release/solidinserts.wasm" -o "$package/plugin.wasm"
wasm-tools validate "$package/plugin.wasm"
cp plugin.json "$package/"
cp LICENSE "$package/"
cp wit/LICENSE-MIT "$package/LICENSE-SOLIDCRAFT-WIT"

python3 - <<'PY'
import hashlib
import json
from pathlib import Path
from shutil import copyfile
from zipfile import ZIP_DEFLATED, ZipFile

package = Path("dist/solidinserts")
manifest = json.loads((package / "plugin.json").read_text(encoding="utf-8"))
series = manifest["runtime"]["entries"][0]["input_schema"]["properties"]["serie"]["oneOf"]
images = [option["x-solidcraft-icon"] for option in series]
for image in images:
    destination = package / image
    destination.parent.mkdir(parents=True, exist_ok=True)
    copyfile(image, destination)

archive = package.parent / f"solidinserts-{manifest['version']}.zip"
with ZipFile(archive, "w", ZIP_DEFLATED) as output:
    for name in ("plugin.json", "plugin.wasm", "LICENSE", "LICENSE-SOLIDCRAFT-WIT", *images):
        output.write(package / name, f"{package.name}/{name}")
checksum = hashlib.sha256(archive.read_bytes()).hexdigest()
archive.with_suffix(".zip.sha256").write_text(f"{checksum}  {archive.name}\n", encoding="ascii")
print(archive)
PY
