#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"

python3 - <<'PY'
import hashlib
import json
from pathlib import Path
from zipfile import ZIP_DEFLATED, ZipFile

root = Path(".")
version = json.loads((root / "plugin.json").read_text(encoding="utf-8"))["version"]
plugin = root / "dist" / f"solidinserts-{version}.zip"
plugin_checksum = plugin.with_suffix(".zip.sha256")
if not plugin.is_file() or not plugin_checksum.is_file():
    raise SystemExit("Run ./build.sh to produce the versioned plugin ZIP and checksum.")
if hashlib.sha256(plugin.read_bytes()).hexdigest() != plugin_checksum.read_text(encoding="ascii").split()[0]:
    raise SystemExit(f"Checksum mismatch: {plugin}")

files = (
    plugin,
    plugin_checksum,
    "README.md",
    "README.it.md",
    "NOTICE.md",
    "LICENSE",
    "Cargo.toml",
    "Cargo.lock",
    "plugin.json",
    "build.sh",
    "package-preview-kit.sh",
    "prova-select-holes.sh",
    "tests/solidcraft.py",
    "wit/LICENSE-MIT",
    "wit/plugin.wit",
    *sorted((root / "src").glob("*.rs")),
    "docs/desktop-preview.md",
    "solidcraft-v0.41.0-select-holes.patch",
    "examples/provino-fori-pilota.sc",
    "examples/provino-sedi-inserti.sc",
    "assets/solidinserts/standard.png",
    "assets/solidinserts/short.png",
    "assets/solidinserts/voron.png",
    "assets/screenshots/insert-seat-preview.png",
    "assets/branding/solidinserts-logo.svg",
    "assets/branding/solidinserts-logo-inverse.svg",
)
name = f"solidinserts-{version}-preview-kit"
kit = root / "dist" / f"{name}.zip"
staged = kit.with_suffix(".zip.tmp")
try:
    with ZipFile(staged, "w", ZIP_DEFLATED) as archive:
        for item in map(Path, files):
            path = item.name if item.parent == Path("dist") else item.as_posix()
            archive.write(item, f"{name}/{path}")
    staged.replace(kit)
finally:
    staged.unlink(missing_ok=True)
digest = hashlib.sha256(kit.read_bytes()).hexdigest()
kit.with_suffix(".zip.sha256").write_text(f"{digest}  {kit.name}\n", encoding="ascii")
print(kit)
PY
