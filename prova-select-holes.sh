#!/usr/bin/env bash
set -euo pipefail

workspace="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
binary="$workspace/dist/anteprima-linux/solidcraft-select-run"
package="$workspace/dist/solidinserts"
if [[ ! -x "$binary" || ! -r "$package/plugin.wasm" ]]; then
    printf '%s\n' "Compila l’app e il plugin seguendo le istruzioni in README.md." >&2
    exit 1
fi

# A fresh profile keeps the desktop installation and its files separate.
profile="$(mktemp -d "${TMPDIR:-/tmp}/solidinserts-preview.XXXXXXXX")"
trap 'rm -rf -- "$profile"' EXIT
export XDG_DATA_HOME="$profile/data"
export XDG_CACHE_HOME="$profile/cache"
export XDG_CONFIG_HOME="$profile/config"
export SOLIDCRAFT_SETTINGS_DIR="$profile/settings"
export SOLIDCRAFT_RECOVERY_DIR="$XDG_DATA_HOME/solidcraft/recovery"
unset SOLIDCRAFT_CONTROL_PORT SOLIDCRAFT_CONTROL_TOKEN SOLIDCRAFT_CONTROL_DIR
unset SOLVECRAFT_CONTROL_PORT SOLVECRAFT_CONTROL_TOKEN SOLVECRAFT_CONTROL_DIR
unset APPIMAGE APPDIR

mkdir -p "$XDG_DATA_HOME/solidcraft/plugins" "$SOLIDCRAFT_SETTINGS_DIR" "$XDG_CACHE_HOME" "$XDG_CONFIG_HOME"
cp -a -- "$package" "$XDG_DATA_HOME/solidcraft/plugins/solidinserts"
cat > "$SOLIDCRAFT_SETTINGS_DIR/app.ron" <<'RON'
{"solidcraft.prefs":"{\"preferences\":{\"language\":\"it\",\"agent_control\":false,\"agent_connect_at_start\":false,\"agent_chat_open_at_start\":false,\"check_updates\":false}}"}
RON

# Preserve the pilot. The working copy and any saves to it stay under dist/.
example="$(mktemp --suffix=.sc "$workspace/dist/provino-select-holes-XXXXXX")"
cp -- "$workspace/examples/provino-fori-pilota.sc" "$example"
printf 'Provino di lavoro: %s\n' "$example"
"$binary" "$example"
