#!/usr/bin/env python3
"""Pack PNG images into a Windows `.ico` or a macOS `.icns`.

Pure standard library: no third-party tools, no network. `packaging/icons.sh` renders the PNGs
with resvg or rsvg-convert and calls this to assemble the container formats the two operating
systems need: Windows reads a `.ico`, macOS a `.icns`.

  gen-icons.py ico  <out.ico>  <in.png>...
  gen-icons.py icns <out.icns> <in.png>...

Each input PNG must be square. For ICNS the pixel size picks the element type, mirroring what
`iconutil` writes from an `.iconset`.
"""

import struct
import sys
from pathlib import Path


def png_size(data: bytes) -> tuple[int, int]:
    # PNG: 8-byte signature, then the IHDR chunk: length(4) type(4) width(4) height(4).
    if data[:8] != b"\x89PNG\r\n\x1a\n":
        raise SystemExit("not a PNG file")
    width, height = struct.unpack(">II", data[16:24])
    return width, height


def pack_ico(out: Path, pngs: list[Path]) -> None:
    images = []
    for p in sorted(pngs, key=lambda p: png_size(p.read_bytes())[0]):
        data = p.read_bytes()
        w, h = png_size(data)
        if w != h or w > 256:
            raise SystemExit(f"{p}: icon images must be square and at most 256 px, got {w}x{h}")
        images.append((w, data))
    header = struct.pack("<HHH", 0, 1, len(images))  # reserved, type=icon, count
    body = b""
    offset = 6 + 16 * len(images)
    directory = b""
    for w, data in images:
        dim = 0 if w == 256 else w  # 256 is encoded as 0
        directory += struct.pack("<BBBBHHII", dim, dim, 0, 0, 1, 32, len(data), offset)
        body += data
        offset += len(data)
    out.write_bytes(header + directory + body)


# ICNS element types by pixel size, as iconutil emits from an .iconset (PNG payloads).
ICNS_TYPES = {
    16: b"icp4",
    32: b"icp5",
    64: b"icp6",
    128: b"ic07",
    256: b"ic08",
    512: b"ic09",
    1024: b"ic10",
}


def pack_icns(out: Path, pngs: list[Path]) -> None:
    chunks = b""
    for p in sorted(pngs, key=lambda p: png_size(p.read_bytes())[0]):
        data = p.read_bytes()
        w, h = png_size(data)
        if w != h:
            raise SystemExit(f"{p}: icon images must be square, got {w}x{h}")
        if w not in ICNS_TYPES:
            raise SystemExit(f"{p}: no ICNS element type for {w} px")
        chunk = ICNS_TYPES[w] + struct.pack(">I", len(data) + 8) + data
        chunks += chunk
    total = 8 + len(chunks)
    out.write_bytes(b"icns" + struct.pack(">I", total) + chunks)


def main(argv: list[str]) -> int:
    if len(argv) < 3 or argv[0] not in ("ico", "icns"):
        print(__doc__, file=sys.stderr)
        return 2
    kind, out = argv[0], Path(argv[1])
    pngs = [Path(p) for p in argv[2:]]
    if kind == "ico":
        pack_ico(out, pngs)
    else:
        pack_icns(out, pngs)
    print(f"{out}: {len(pngs)} images")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
