#!/usr/bin/env bash
# Regenerate every derived app icon from assets/app-icon/trackline.svg (the canonical artwork,
# derived from the fork's plan/icon.svg). trackline-small.svg is the same drawing kept small and
# is copied, not generated.
#
# Needs: resvg (brew install resvg / cargo install resvg) or rsvg-convert (librsvg). The .ico and
# .icns are assembled by packaging/gen-icons.py (standard library only); on macOS, `iconutil` is
# used for the .icns when present. The outputs are committed, so builds and packaging never need
# these tools.
#
#   packaging/icons.sh
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIR="$ROOT/assets/app-icon"
SVG="$DIR/trackline.svg"
APP_ID="ru.illoprin.trackline"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

render() {
  local input="$1" size="$2" output="$3"
  if command -v resvg >/dev/null; then
    resvg -w "$size" -h "$size" "$input" "$output" </dev/null
  else
    rsvg-convert -w "$size" -h "$size" "$input" -o "$output" </dev/null
  fi
}
command -v resvg >/dev/null || command -v rsvg-convert >/dev/null || {
  echo "error: need resvg (brew install resvg) or rsvg-convert (librsvg)" >&2
  exit 1
}

# macOS icons follow Apple's grid: the tile body is 824 of 1024 px with a transparent margin, so
# the Dock shows it at the same size as other apps. The Trackline artwork already carries its own
# margin inside the 512 viewBox, so widen the viewBox to place the tile at the grid size.
MAC="$TMP/mac.svg"
cat >"$MAC" <<EOF
<svg width="512" height="512" viewBox="-21.02 -21.02 554.04 554.04" fill="none" xmlns="http://www.w3.org/2000/svg">
EOF
tail -c +2 "$SVG" >>"$MAC"
grep -q 'viewBox="-21.02' "$MAC" || { echo "error: unexpected viewBox in $SVG" >&2; exit 1; }

render "$SVG" 1024 "$DIR/trackline-1024.png"
# Runtime window/Dock icon on macOS (apps/filmcraft/src/main.rs embeds it).
render "$MAC" 512 "$DIR/trackline-macos-512.png"

# Linux hicolor theme. Windows and Linux use the full-bleed tile.
for s in 16 24 32 48 64 128 256 512; do
  mkdir -p "$DIR/hicolor/${s}x${s}/apps"
  render "$SVG" "$s" "$DIR/hicolor/${s}x${s}/apps/$APP_ID.png"
done
mkdir -p "$DIR/hicolor/scalable/apps"
cp "$DIR/trackline-small.svg" "$DIR/hicolor/scalable/apps/$APP_ID.svg"

# Web app favicon and loading image.
render "$SVG" 128 "$ROOT/apps/filmcraft-web/web/favicon.png"

# Windows .ico.
ICO_PNGS=()
for s in 16 20 24 32 40 48 64 128 256; do
  render "$SVG" "$s" "$TMP/ico-$s.png"
  ICO_PNGS+=("$TMP/ico-$s.png")
done
python3 "$ROOT/packaging/gen-icons.py" ico "$DIR/trackline.ico" "${ICO_PNGS[@]}"

# macOS .icns: iconutil on macOS, else the bundled packer.
if command -v iconutil >/dev/null; then
  SET="$TMP/trackline.iconset"
  mkdir -p "$SET"
  for s in 16 32 128 256 512; do
    render "$MAC" "$s" "$SET/icon_${s}x${s}.png"
    render "$MAC" $((s * 2)) "$SET/icon_${s}x${s}@2x.png"
  done
  iconutil -c icns -o "$DIR/trackline.icns" "$SET"
else
  ICNS_PNGS=()
  for s in 16 32 64 128 256 512; do
    render "$MAC" "$s" "$TMP/icns-$s.png"
    ICNS_PNGS+=("$TMP/icns-$s.png")
  done
  render "$MAC" 1024 "$TMP/icns-1024.png"
  ICNS_PNGS+=("$TMP/icns-1024.png")
  python3 "$ROOT/packaging/gen-icons.py" icns "$DIR/trackline.icns" "${ICNS_PNGS[@]}"
fi
echo "icons written to $DIR"
