<p align="center">
  <img src="assets/app-icon/hicolor/256x256/apps/ru.illoprin.trackline.png" width="140" height="140" alt="Trackline icon: a white &quot;Tr&quot; monogram on a dark-purple rounded tile">
</p>

<h1 align="center">Trackline</h1>

<p align="center">
  <b>A non-linear video editor and all-in-one toolkit for social media.</b><br>
  Cut, grade, mix, caption and export — in one fast native app, with an automation interface for agents.
</p>

<p align="center">
  <a href="README.md">English</a> · <a href="README.ru.md">Русский</a>
</p>

<p align="center">
  <a href="#license-and-credits"><img alt="License: MIT OR Apache-2.0" src="https://img.shields.io/badge/license-MIT%20%7C%20Apache--2.0-8b5cf6?style=flat-square"></a>
  <img alt="Written in Rust" src="https://img.shields.io/badge/language-Rust-6a3fd6?style=flat-square&logo=rust&logoColor=white">
  <img alt="Fork of FilmCraft" src="https://img.shields.io/badge/fork%20of-FilmCraft-6a3fd6?style=flat-square">
</p>

---

> [!IMPORTANT]
> **Trackline is an independent fork of FilmCraft.** It is based on the original project
> **[storytold/filmcraft](https://github.com/storytold/filmcraft)** by the ArtCraft Team and the
> FilmCraft contributors. Trackline is **not** an official FilmCraft or ArtCraft product and is not
> made, sponsored, endorsed or supported by the ArtCraft Team. All upstream code, licences and
> notices are kept — see [License and credits](#license-and-credits).

Trackline is a professional-style non-linear editor aimed at people who cut for the web: familiar
panels, workspaces, tools and shortcuts, so your hands already know where everything is. Underneath
it is a from-scratch Rust engine — its own H.264, HEVC, ProRes and AAC codecs, a GPU compositor that
blends in linear light, integer tick time so edits never drift, and an engine where every action is a
command an AI agent can drive as precisely as you can.

<!-- TODO (media to add when Trackline screenshots exist): this README should ship with fresh
     Trackline-branded screenshots — the Spectrum Analyzer on the master bus; Effect Controls with
     selected keyframes and copied effects; the Audio Track Mixer with saved track presets; the
     timeline with audio fades and rounded clip plates; and the About dialog with the Trackline
     logo. Until then the previews below are the upstream FilmCraft documentation screenshots (the
     same engine), kept so the page has real visuals; they carry the upstream brand and
     public-domain footage credited in ATTRIBUTION.md. -->

> [!NOTE]
> The previews below come from the upstream FilmCraft documentation (it is the same engine).
> Trackline-branded screenshots are pending; each image is credited in [ATTRIBUTION.md](ATTRIBUTION.md).

---

## What Trackline adds

Trackline keeps FilmCraft's engine and adds the workflow tools an editor reaches for every day.

### Spectrum Analyzer on the master bus

A live spectrum analyzer draws the frequency response of everything arriving at the master,
**after** the master effects, so you see exactly what leaves the mixer. Use it to catch muddy
low-mids, find a resonant ring, or set a de-esser by eye while the sequence plays.

<p align="center">
  <img src="docs/images/filmcraft-effects.png" alt="Effect Controls and the Effects browser; a Trackline spectrum analyzer traces the master bus frequency response" width="100%">
</p>

<p align="center"><sub><b>See the mix, not just the meters.</b> The spectrum analyzer follows the master bus through its effects.</sub></p>

### New effects, video and audio

- **Camera Shake** (video): procedural handheld shake with amount, frequency and rotation.
- **Gate** (audio): a full noise gate with Threshold, Attack, Release, Hold, Floor and Gain, a
  sidechain section (Floor, Highpass, Lowpass) and an **Inverse** switch to hear what the gate cuts.
- **Bitcrusher** and **Retro Wiggle** (audio): lo-fi bit reduction and tape-style pitch wobble.
- Plus a reworked colour picker with a **hex text field** you can copy and paste.

<p align="center">
  <img src="docs/images/filmcraft-timeline.png" alt="Timeline close-up: clips with rounded plates, an audio fade ramp drawn on a clip, coloured markers and loudness meters" width="100%">
</p>

<p align="center"><sub><b>Rounded clip plates and Reaper/Vegas-style audio fades.</b> Drag the triangles at a clip's edges for a smooth audio ramp — drawn right on the waveform.</sub></p>

### Effect Controls, built for speed

- **Select and edit many keyframes at once** — marquee-select, then move, delete (`Del`) or change
  interpolation for all of them in one step. `Alt`+drag duplicates keyframes.
- **Copy and paste effects and keyframes** (`Ctrl+C` / `Ctrl+V`) between clips, tracks and even
  sequences. Paste onto a selection and the effects land where you want; paste keyframes and a
  missing effect is applied for you, relative to the playhead.
- **Reorder effects by dragging**, with a clear accent insertion bar and a hand cursor, exactly
  like reordering tabs.
- **Keyframe pills** on a collapsed effect show its animation at a glance, like Premiere Pro.
- **Alt+wheel zooms the keyframe lane to the playhead**, and the numeric drag locks to the cursor so
  a big value change no longer stops at the window edge.

<p align="center">
  <img src="docs/images/filmcraft-keyframes.png" alt="Effect Controls with value and velocity graphs for an eased keyframe, several keyframes selected, and a copy/paste menu on the keyframe lane" width="100%">
</p>

<p align="center"><sub><b>Keyframes as objects.</b> Select many, change interpolation in bulk, copy them to another clip, and reorder effects by drag.</sub></p>

### Audio Track Mixer: presets and track-to-track effects

- **Save a whole track as a preset** with a name and a description, and apply it to any track.
- **Copy effects from one track and paste them onto another** — across tracks and sequences — through
  the shared clipboard, with as many slots filled as fit.
- **Delete all effects on a track** in one click.

### Timeline quality of life

- **Audio fades** with an invisible boundary line: before it you edit volume, after it you edit the
  transition; drag the boundary to resize (shorter than one frame removes the fade), `Alt` for an
  exponential fade, and `Del` removes a selected audio transition.
- **Trim several selected clips at once** — trim left/right or change length applies to the whole
  selection.
- **Paste takes the target tracks into account**, not the tracks you copied from.
- **Clip plates** with 4 px rounded corners and a subtle inset highlight, masked so waveforms and
  thumbnails clip to the rounding.

### Text styles as project assets

- **Reusable text styles** live in the Project panel (an `ItemKind::TextStyle` with a `T` icon),
  export and import as `.fstyle`, and apply to any number of text objects through the context menu,
  the timeline clip menu, drag-and-drop, or the inspector — with **Update Style** (push) and
  **Unlink Style**.
- **Double-click selects a word** when editing text, a font **search box** filters the font list, and
  **copy/paste layers** works in Essential Graphics.

### Linux desktop fixes

- **Drag-and-drop from the file manager works on Wayland**, and pasting a file from the clipboard no
  longer appends a stray carriage return to the path.

---

## Boring technical details

### Formats and codecs

No FFmpeg inside. The video codecs, AAC, Opus and the containers are our own Rust code, implemented
from the public ITU-T, ISO and IETF specifications and tested frame by frame against ffmpeg as an
external oracle.

| | Decode | Encode | Notes |
|---|:---:|:---:|---|
| **H.264 / AVC** | ✓ | ✓ | Decoder bit-exact on 37+ streams; encoder with High/Main/Baseline, CABAC, B-frames, CRF/CBR/VBR/2-pass |
| **HEVC / H.265** | ✓ | | Main and Main 10, tiles, WPP, PCM, long-term references |
| **Apple ProRes** | ✓ | ✓ | Decodes 422 Proxy to 4444 XQ; exports 422 HQ |
| **AAC-LC** | ✓ | ✓ | |
| **VP8 / VP9** | ✓ | | Profiles, 8/10/12-bit, alpha WebM, tiles and superframes |
| **AV1** | ✓ | | Main profile, 8/10-bit 4:2:0 and monochrome, film grain and superres |
| **Opus** | ✓ | | SILK, CELT, hybrid and multistream surround |
| **DNxHD / DNxHR** | ✓ | ✓ | SMPTE ST 2019-1, 8/10/12-bit; exports DNxHR in MOV |
| **MJPEG, PCM** | ✓ | ✓ | |
| **MP3, FLAC, ALAC, Vorbis** | ✓ | | Via the [symphonia](https://github.com/pdeljanov/Symphonia) crate, to be replaced by our own |
| **MP4 / MOV** | ✓ | ✓ | Fragmented MP4, edit lists, timecode tracks |
| **Matroska / WebM** | ✓ | | Lacing, Cues, HDR colour metadata |
| **AVI** | ✓ | | AVI 1.0 and OpenDML; MJPEG, H.264, HEVC and uncompressed RGB/YUV with PCM, MP3, MP2 or AC-3 |
| **Stills** | ✓ | ✓ | PNG, JPEG, GIF, WebP, TIFF and BMP; export PNG sequences and animated GIF |

Also imported: MPEG-2/MPEG-1, AC-3, MP2, MXF (OP1a/OP-Atom), MPEG transport and program streams,
Ogg and image sequences. Hardware decoding on macOS (VideoToolbox), Windows (Media Foundation) and,
for H.264/HEVC, Linux (VA-API); H.264 export can use NVIDIA's encoder, and H.265 export is
hardware-only (NVENC on Windows, VideoToolbox on macOS).

### Export and interchange

- **H.264 MP4 with AAC, using our own encoders**, plus Apple ProRes 422 HQ and Motion JPEG in
  QuickTime, MXF OP1a/OP-Atom, PNG sequences, animated GIF and WAV. Background jobs with progress
  and cancel; content-addressed render previews.
- **Interchange:** FCP 7 XML (xmeml), FCPXML 1.9–1.11, OpenTimelineIO, CMX 3600 EDL, AAF (Edit
  Protocol) and OMF 2.0.
- Trackline's own project files use the FilmCraft `.fcproj` format, so existing FilmCraft projects
  open unchanged.

### Automation

Every menu item, button, slider and drag is a **command** with an id, typed parameters and an
enabled state. There are more than 650 engine commands (`trackline-cli commands` lists them). The
UI, the CLI, a JSON control channel and an **MCP server** all dispatch the same commands, so Claude
or any agent can cut, trim, grade, mix and export exactly the way a person does.

```jsonc
// over the control channel (JSON lines on TCP) or as MCP tool calls
{"method": "engine.execute", "params": {"command": "timeline.place",
  "params": {"item": 1, "track": "V1", "seconds": 0, "sourceIn": 284298240000000, "duration": 1270080000000}}}
{"method": "ui.screenshot", "params": {"path": "frame.png"}}
```

### Build from source

Trackline builds with a recent stable Rust toolchain (edition 2024; see `rust-version` in
`Cargo.toml`).

```sh
git clone https://github.com/illoprin/trackline
cargo run --release -p filmcraft                    # the desktop app (binary: trackline), with a demo project
cargo run --release -p filmcraft -- --control 9876  # plus the JSON-lines control server
cargo run --release -p filmcraft-cli -- commands     # list every engine command (binary: trackline-cli)
cargo run --release -p filmcraft-cli -- mcp          # MCP server (headless)
```

Japanese text in the interface and in titles comes from
[craft-fonts](https://github.com/storytold/craft-fonts), an optional build input:

```sh
git clone https://github.com/storytold/craft-fonts ../craft-fonts
CRAFT_FONTS_DIR="$PWD/../craft-fonts" cargo run --release -p filmcraft
```

The control protocol is documented in [docs/control-protocol.md](docs/control-protocol.md).

> Trackline does not publish prebuilt binaries yet. The original FilmCraft's releases are on its
> [releases page](https://github.com/storytold/filmcraft/releases), but those are FilmCraft builds.

### Documentation

| | |
|---|---|
| [AGENTS.md](AGENTS.md) | The rules every contributor follows: assets, clean room, licences |
| [docs/architecture.md](docs/architecture.md) | Layers, data model, time base, command system, render and export pipeline |
| [docs/testing.md](docs/testing.md) | Unit, property and ffmpeg-oracle tests, accuracy criteria, benchmarks |
| [docs/agents.md](docs/agents.md) | Driving the app over MCP and the control channel |
| [docs/project-files.md](docs/project-files.md) | `.fcproj` format, schema migrations, auto-save and crash recovery |

### Status

Trackline inherits FilmCraft's engine, which is in **alpha** and moving fast. Editing, trimming,
multicam, colour, keyframes, effects, titles, captions, mixing, codecs and export work today. Known
gaps: no native `.prproj` import, no plugin hosting, H.265 and AV1 export are hardware-only (or
absent), and camera/phone media is less tested than conformance streams.

## License and credits

Trackline is dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
Copyright (c) 2026 the Trackline fork contributors; based on FilmCraft, Copyright (c) 2026 ArtCraft
Team and the FilmCraft contributors. Required notices are in [NOTICE](NOTICE).

Bundled fonts, icons, images and other assets keep their own open licences; each one is listed with
its author, source and licence in [ATTRIBUTION.md](ATTRIBUTION.md).

The ArtCraft name, wordmark and logos in [`docs/brand/`](docs/brand/) are trademarks of the ArtCraft
Team and are **not** covered by this licence. This fork does not use them; they are retained only as
upstream provenance under [`docs/brand/LICENSE-brand.txt`](docs/brand/LICENSE-brand.txt).

Trackline is an independent implementation. It contains no proprietary code, icons, images, presets
or LUTs, and no GPL or LGPL code; every icon is drawn in code and every asset is openly licensed and
attributed ([AGENTS.md](AGENTS.md)). ffmpeg is used only as an external test oracle.
