# Trackline app icon

**Creature:** the letters "Tr" in a rounded dark-purple tile — the Trackline mark. It is the
fork's own artwork, supplied as `plan/icon.svg` and derived into every platform file here.

## Palette

| Colour | Hex | Used for |
|---|---|---|
| Tile | `#2f064e` | the rounded-square field |
| Letter | `#ffffff` at 50 % | the "Tr" monogram |
| Shadow | `#000000` at 36 % / 25 % | the tile and monogram drop shadows |

## Geometry and style

- A 512-unit viewBox tile with the app colour, rounded corners and a small transparent margin
  (the tile body runs from 33 to 479 units).
- The "Tr" monogram is white at half opacity with a soft drop shadow.
- macOS renders (`trackline.icns`, `trackline-macos-512.png`) put the tile in Apple's grid
  (824 of 1024 px, transparent margin). Windows, Linux and the web use the tile as drawn.

## Provenance

The artwork was supplied for the Trackline fork as `plan/icon.svg`. Licence: [LICENSE.txt](LICENSE.txt) (MIT OR Apache-2.0).

## Files

| File | What |
|---|---|
| `trackline.svg` | canonical artwork: `plan/icon.svg` with the drop shadows written as `feDropShadow` so librsvg and resvg both render it |
| `trackline-small.svg` | the same drawing kept as the small/scalable variant |
| `trackline-1024.png` | 1024 px render |
| `trackline-macos-512.png` | runtime Dock icon on macOS (embedded by `apps/filmcraft/src/main.rs`) |
| `trackline.icns` | macOS bundle icon (the bundle's `CFBundleIconFile` is `Trackline`) |
| `trackline.ico` | Windows icon, 16–256 px (embedded in `trackline.exe` by `apps/filmcraft/build.rs`) |
| `hicolor/<size>/apps/ru.illoprin.trackline.png`, `hicolor/scalable/…svg` | Linux icon theme, used by `packaging/linux/ru.illoprin.trackline.desktop` |

The web app's `apps/filmcraft-web/web/favicon.png` (128 px) comes from the same artwork.

## How the icon reaches each OS

- **macOS:** the window icon set at startup (`ViewportBuilder::with_icon`) is the Dock and app-switcher
  icon when the app runs unbundled. The `.app` bundle points `CFBundleIconFile` at `Trackline`, which
  `packaging/macos/package.sh` fills with `trackline.icns`.
- **Windows:** `build.rs` embeds `trackline.ico` as the exe's resource icon (Explorer, Start menu,
  pinned taskbar), and the runtime icon covers the window, taskbar and Alt-Tab.
- **Linux:** the runtime icon plus `with_app_id("ru.illoprin.trackline")`, which Wayland matches to
  the `.desktop` file. Install the `hicolor/` tree under `/usr/share/icons/hicolor/` and the desktop
  file under `/usr/share/applications/`.

## Regenerate

`packaging/icons.sh` (needs resvg or rsvg-convert; `iconutil` on macOS for the `.icns`, otherwise
the bundled `packaging/gen-icons.py`). To change the artwork, edit `plan/icon.svg` and regenerate,
then run the script. Every derived file needs its `.attribution` sidecar and a row in
`ATTRIBUTION.md`.
