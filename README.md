<p align="center">
  <img src="assets/app-icon/hicolor/256x256/apps/ru.illoprin.trackline.png" width="140" height="140" alt="Trackline icon: a white &quot;Tr&quot; monogram on a dark-purple rounded tile">
</p>

<h1 align="center">Trackline</h1>

<p align="center">
  <b>A non-linear video editor and all-in-one toolkit for social media.</b><br>
  Cut, grade, mix, caption and export — in one fast native app, with an automation interface for agents.
</p>

<p align="center">
  <a href="#license-and-credits"><img alt="License: MIT OR Apache-2.0" src="https://img.shields.io/badge/license-MIT%20%7C%20Apache--2.0-8b5cf6?style=flat-square"></a>
  <img alt="Written in Rust" src="https://img.shields.io/badge/language-Rust-6a3fd6?style=flat-square&logo=rust&logoColor=white">
  <img alt="Fork of FilmCraft" src="https://img.shields.io/badge/fork%20of-FilmCraft-6a3fd6?style=flat-square">
</p>

<p align="center">
  <a href="#edit">Edit</a> ·
  <a href="#color">Color</a> ·
  <a href="#effects">Effects</a> ·
  <a href="#audio">Audio</a> ·
  <a href="#titles-and-captions">Titles</a> ·
  <a href="#formats">Formats</a> ·
  <a href="#export">Export</a> ·
  <a href="#automation">Automation</a> ·
  <a href="#about-this-fork">About this fork</a> ·
  <a href="#build-from-source">Build</a> ·
  <a href="#license-and-credits">License</a>
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
it is a from-scratch Rust engine: its own H.264, HEVC, ProRes and AAC codecs, a GPU compositor that
blends in linear light, integer tick time so edits never drift, and an engine where every action is
a command an AI agent can drive as precisely as you can.

<!-- TODO (media to add when Trackline screenshots exist): this README should ship with fresh
     Trackline-branded screenshots — a wide hero shot of the Edit workspace mid-cut, a Color
     workspace with scopes, an Effects/Effect Controls view with keyframes, the Audio mixer with
     loudness meters, the Export mode with a social-media preset, and the About dialog showing the
     Trackline logo. Until then the previews below are the upstream FilmCraft documentation
     screenshots (the same engine), kept so the page has real visuals; they carry the upstream
     brand and public-domain footage credited in ATTRIBUTION.md. -->

> [!NOTE]
> The previews below come from the upstream FilmCraft documentation (it is the same engine).
> Trackline-branded screenshots are pending; each image is credited in [ATTRIBUTION.md](ATTRIBUTION.md).

---

## Edit

<p align="center">
  <img src="docs/images/filmcraft-assembly.png" alt="Assembly workspace: bins of film thumbnails, a Source monitor, a Program monitor, and a three-track timeline with music and loudness meters" width="100%">
</p>

**A timeline you already know.** Source and Program monitors, bins with thumbnails, a multi-track
timeline with patch and target buttons, sync locks and linked selection, and dockable panels you
can arrange into any workspace.

- **Three-point editing.** Mark In and Out in the Source monitor, then insert or overwrite onto the
  patched tracks. Lift and extract take ranges back out.
- **Every trim.** Ripple, roll, slip, slide, rate stretch and razor. Dedicated Trim and Slip
  Monitors, dynamic trimming while it plays, and ripple-trim to the playhead.
- **Exact time.** Every edit is computed on integer ticks (254,016,000,000 per second), which
  divides evenly by every common frame rate and sample rate — 23.976, 29.97 drop-frame and 59.94 are
  exact, not approximate.
- **The details that matter.** Markers with colours and durations, nesting, copy/paste and paste
  insert, ripple delete and close gap, snapping, and unlimited undo with a History panel.
- **Never lose work.** Atomic saves, rolling auto-save versions, and a crash-recovery journal that
  brings back unsaved changes after a crash or power cut.

## Color

<p align="center">
  <img src="docs/images/filmcraft-color.png" alt="Color workspace: a Program monitor with live waveform and vectorscope scopes beside it" width="100%">
</p>

A complete color grading panel in the order a colorist works:

- **Basic Correction:** temperature and tint, exposure, contrast, highlights, shadows, whites,
  blacks, saturation.
- **Creative:** built-in looks with an intensity control, plus faded film, sharpen and vibrance.
- **Curves:** an RGB curve editor with monotone-cubic interpolation, plus hue-vs-saturation,
  hue-vs-hue, hue-vs-luma, luma-vs-saturation and sat-vs-sat.
- **Color Wheels:** shadow, midtone and highlight wheels, each with a lightness control.
- **HSL Secondary:** key by hue, saturation and luma, view the key as a matte, correct only what you
  selected. Plus a vignette with amount, midpoint, roundness and feather.

All grading happens in linear light, in 32-bit float, and the waveform and vectorscope are computed
from the graded frame.

## Effects

<p align="center">
  <img src="docs/images/filmcraft-effects.png" alt="Effects workspace: Effect Controls with Motion, Opacity and a colour grade, and the Effects browser open on transitions" width="100%">
</p>

- **A large Video Effects bin**, legacy effects, and dozens of transitions: blurs, keys, distortions,
  glows, immersive video effects, Posterize Time, Echo, dissolves, wipes, irises, pushes, slides,
  zooms and more.
- **Motion and opacity on every clip:** position, scale, rotation, anchor point, anti-flicker and
  blend modes.
- **Advanced keyframing:** linear, Bezier, auto/continuous Bezier, hold, ease in and ease out, with
  value and velocity graphs and a shared keyframe navigator between Effect Controls and Properties.
- **A GPU compositor** built on wgpu (Metal, Vulkan, DirectX 12, WebGPU) that samples YUV straight
  from the decoder and blends in linear light, with a CPU path tested against it.

## Audio

- **Loudness meters to broadcast standards:** momentary, short-term and integrated loudness and true
  peak to ITU-R BS.1770 and EBU R128, alongside peak meters.
- **Clip effects on our own DSP:** parametric EQ, high-/low-/band-pass filters, dynamics, a true-peak
  limiter, delay, reverb, DeNoise, DeHummer, invert and pitch shift, all keyframable.
- **Audio Track and Clip Mixers:** faders, pan, mute, solo, insert slots, sends, submixes and a Mix
  track, sample-accurate and latency-compensated, with read/latch/touch/write automation.
- **Audio-clock playback:** the sound card is the master clock, so picture follows sound.

## Titles and captions

- **A real text engine:** OpenType shaping with kerning and ligatures, bidirectional text, line
  breaking, tracking and leading — sharp at any scale or rotation.
- **Type, Shape and Pen tools** on the Program monitor, edited in the Properties panel and
  keyframable.
- **Captions:** caption tracks in Subtitle, CEA-608, CEA-708 and Teletext formats; import and export
  SRT, WebVTT and SCC with frame-exact timing, and burn captions into exports.

## Formats

No FFmpeg inside. The video codecs, AAC, Opus and the containers are our own Rust code, implemented
from the public ITU-T, ISO and IETF specifications and tested frame by frame against ffmpeg as an
external oracle.

| | Decode | Encode | Notes |
|---|:---:|:---:|---|
| **H.264 / AVC** | ✓ | ✓ | Decoder bit-exact on 37+ streams; encoder with High/Main/Baseline profiles, CABAC, B-frames, CRF/CBR/VBR/2-pass |
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
| **AVI** | ✓ | | AVI 1.0 and OpenDML; Motion JPEG, H.264, HEVC and uncompressed RGB/YUV with PCM, MP3, MP2 or AC-3 |
| **Stills** | ✓ | ✓ | PNG, JPEG, GIF, WebP, TIFF and BMP (animated GIF/APNG/WebP as clips); export PNG sequences and animated GIF |

Also imported: MPEG-2/MPEG-1, AC-3, MP2, MXF (OP1a/OP-Atom), MPEG transport and program streams,
Ogg and image sequences. Hardware decoding works on macOS (VideoToolbox), Windows (Media Foundation)
and, for H.264/HEVC, Linux (VA-API); H.264 export can use NVIDIA's encoder, and H.265 export is
hardware-only (NVENC on Windows, VideoToolbox on macOS).

## Export

<p align="center">
  <img src="docs/images/filmcraft-export.png" alt="Export mode: social destination presets on the left, a preview in the middle and H.264 settings on the right" width="100%">
</p>

- **H.264 MP4 with AAC, using our own encoders**, plus Apple ProRes 422 HQ and Motion JPEG in
  QuickTime, MXF OP1a/OP-Atom, PNG sequences, animated GIF and WAV.
- **Background jobs** with progress and cancel, so you keep editing while it renders.
- **Render previews:** the render bar marks segments green, yellow or red, and rendered previews are
  cached by content, so an edit only invalidates what it touches.

## Interchange

Move timelines between Trackline and other editors:

- **FCP 7 XML (xmeml)** and **FCPXML 1.9–1.11**.
- **OpenTimelineIO** (round-trips through `metadata.filmcraft`).
- **CMX 3600 EDL** with drop-frame timecode and dissolves.
- **AAF (Edit Protocol)** and **OMF 2.0** for audio post.

Trackline's own project files use the FilmCraft `.fcproj` format, so existing FilmCraft projects open
unchanged.

## Automation

Every menu item, button, slider and drag is a **command** with an id, typed parameters and an enabled
state. There are more than 650 engine commands (`trackline-cli commands` lists them). The UI, the
CLI, a JSON control channel and an **MCP server** all dispatch the same commands, so Claude or any
agent can cut, trim, grade, mix and export exactly the way a person does. Every widget also has an
automation id, so an agent can click, drag, type and take screenshots.

```jsonc
// over the control channel (JSON lines on TCP) or as MCP tool calls
{"method": "engine.execute", "params": {"command": "timeline.place",
  "params": {"item": 1, "track": "V1", "seconds": 0, "sourceIn": 284298240000000, "duration": 1270080000000}}}
{"method": "ui.screenshot", "params": {"path": "frame.png"}}
```

## About this fork

- **Upstream:** [storytold/filmcraft](https://github.com/storytold/filmcraft) by the ArtCraft Team
  and the FilmCraft contributors. Bug reports about the upstream engine belong upstream; Trackline
  tracks the same project.
- **What Trackline changes:** the product name, application identifier (`ru.illoprin.trackline`), the
  app icon and the packaging, plus a re-branded About/Help experience. The engine, codecs, project
  format and command surface are the upstream ones.
- **Not official:** Trackline is not affiliated with, sponsored by, endorsed by or supported by the
  ArtCraft Team. The ArtCraft name and logos are their trademarks and are not used here (the fork
  brand licence requires forks to remove them).
- **Data locations kept for compatibility:** settings, auto-save and crash recovery stay in the same
  per-user FilmCraft folders (for example `~/.local/share/filmcraft`), so data written by earlier
  builds is found again.

## Build from source

Trackline builds with a recent stable Rust toolchain (edition 2024; see `rust-version` in
`Cargo.toml`).

```sh
git clone https://github.com/storytold/filmcraft   # up-to-date while Trackline has no public repo
cargo run --release -p filmcraft                    # the desktop app, with a demo project
cargo run --release -p filmcraft -- --control 9876  # plus the JSON-lines control server
cargo run --release -p filmcraft-cli -- commands     # list every engine command
cargo run --release -p filmcraft-cli -- mcp          # MCP server (headless)
```

Japanese text in the interface and in titles comes from
[craft-fonts](https://github.com/storytold/craft-fonts), an optional build input:

```sh
git clone https://github.com/storytold/craft-fonts ../craft-fonts
CRAFT_FONTS_DIR="$PWD/../craft-fonts" cargo run --release -p filmcraft
```

The control protocol is documented in [docs/control-protocol.md](docs/control-protocol.md).

> Trackline does not publish prebuilt binaries yet. Upstream FilmCraft's releases are available from
> its [releases page](https://github.com/storytold/filmcraft/releases), but those are FilmCraft
> builds, not Trackline ones.

## Documentation

| | |
|---|---|
| [CONTRIBUTING.md](CONTRIBUTING.md) · [docs/contributing.md](docs/contributing.md) | Setup, quality gates, commit conventions, how to add commands, effects, codecs, panels and assets |
| [AGENTS.md](AGENTS.md) | The rules every contributor follows: assets, clean room, licences |
| [docs/architecture.md](docs/architecture.md) | Layers, data model, time base, command system, render and export pipeline |
| [docs/testing.md](docs/testing.md) | Unit, property and ffmpeg-oracle tests, accuracy criteria, benchmarks |
| [docs/agents.md](docs/agents.md) | Driving the app over MCP and the control channel |
| [docs/project-files.md](docs/project-files.md) | `.fcproj` format, schema migrations, auto-save and crash recovery |

## Status

Trackline inherits FilmCraft's engine, which is in **alpha** and moving fast. Editing, trimming,
multicam, colour, keyframes, effects, titles, captions, mixing, codecs and export work today.

The biggest gaps carried over from upstream:

- **Native `.prproj` project import.** `.prproj` files can't be opened yet; bring sequences over as
  FCP7 XML, AAF or OTIO.
- **No plugins.** No VST3 / Audio Units or OpenFX hosting.
- **Delivery codecs.** H.264 is the only software delivery-codec export; H.265 exports through a
  hardware encoder only; no AV1 export yet.
- **Real-world media.** Decoders are bit-exact on conformance streams, but camera and phone files in
  the wild are less tested.
- **AI features.** Few so far; speech to text is optional and off by default.

## License and credits

Trackline is dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
Copyright (c) 2026 the Trackline fork contributors; based on FilmCraft, Copyright (c) 2026 ArtCraft
Team and the FilmCraft contributors. Required notices are in [NOTICE](NOTICE).

Bundled fonts, icons, images and other assets keep their own open licences; each one is listed with
its author, source and licence in [ATTRIBUTION.md](ATTRIBUTION.md). Builds made with
[craft-fonts](https://github.com/storytold/craft-fonts) also embed its fonts (OFL-1.1).

The ArtCraft name, wordmark and logos in [`docs/brand/`](docs/brand/) are trademarks of the ArtCraft
Team and are **not** covered by this licence. This fork does not use them; they are retained only as
upstream provenance under [`docs/brand/LICENSE-brand.txt`](docs/brand/LICENSE-brand.txt).

Trackline is an independent implementation. It contains no proprietary code, icons, images, presets
or LUTs, and no GPL or LGPL code; every icon is drawn in code and every asset is openly licensed and
attributed ([AGENTS.md](AGENTS.md)). ffmpeg is used only as an external test oracle.

**Footage and music in the upstream screenshots:** NASA's Apollo 11 film and television footage from
[images.nasa.gov](https://images.nasa.gov) (public domain; NASA does not endorse this project);
*Night of the Living Dead* (1968), *Carnival of Souls* (1962) and *Charade* (1963), all in the US
public domain; *Earth Views from the ISS* by NASA; Chopin's Nocturne Op. 48 No. 1 and Ballade No. 1,
performed for Musopen and released under CC0. The media itself is not in this repository. Sources and
details for every asset are in [ATTRIBUTION.md](ATTRIBUTION.md).
