"""Draws the source of the icon: a blue rounded square, corners of a viewfinder and a dot.

Run: python art/icon.py, then npx tauri icon art/icon.png -o src-tauri/icons.
The standard library is enough, so there is no Pillow for the sake of one picture.
"""

import struct
import zlib
from pathlib import Path

SIZE = 1024
HALF = SIZE // 2
BLUE = (47, 111, 223)
WHITE = (255, 255, 255)


def box(x: float, y: float, cx: float, cy: float, hw: float, hh: float, radius: float) -> float:
    """The distance from the point to the rounded rectangle: negative inside."""
    dx = abs(x - cx) - (hw - radius)
    dy = abs(y - cy) - (hh - radius)
    outside = (max(dx, 0.0) ** 2 + max(dy, 0.0) ** 2) ** 0.5
    return outside + min(max(dx, dy), 0.0) - radius


def cover(distance: float) -> float:
    """The share of the pixel covered by the figure, by the distance to its edge."""
    return min(max(0.5 - distance, 0.0), 1.0)


def pixel(x: float, y: float) -> tuple[int, int, int, int]:
    """The colour of a point in the top left quarter of the icon."""
    plate = cover(box(x, y, HALF, HALF, 448, 448, 200))
    mark = max(
        cover(box(x, y, 350, 282, 100, 32, 16)),
        cover(box(x, y, 282, 350, 32, 100, 16)),
        cover(((x - HALF) ** 2 + (y - HALF) ** 2) ** 0.5 - 60),
    )
    red, green, blue = (round(b * (1 - mark) + w * mark) for b, w in zip(BLUE, WHITE))
    return red, green, blue, round(255 * plate)


def quarter() -> list[list[tuple[int, int, int, int]]]:
    """The top left quarter of the icon: the other three are its reflections."""
    return [[pixel(x + 0.5, y + 0.5) for x in range(HALF)] for y in range(HALF)]


def chunk(kind: bytes, body: bytes) -> bytes:
    return struct.pack(">I", len(body)) + kind + body + struct.pack(">I", zlib.crc32(kind + body))


def png(rows: list[list[tuple[int, int, int, int]]]) -> bytes:
    """PNG made of the quarter reflected horizontally and vertically."""
    lines = []
    for row in rows + rows[::-1]:
        full = row + row[::-1]
        lines.append(b"\x00" + bytes(value for dot in full for value in dot))
    return (
        b"\x89PNG\r\n\x1a\n"
        + chunk(b"IHDR", struct.pack(">IIBBBBB", SIZE, SIZE, 8, 6, 0, 0, 0))
        + chunk(b"IDAT", zlib.compress(b"".join(lines), 9))
        + chunk(b"IEND", b"")
    )


if __name__ == "__main__":
    Path(__file__).with_name("icon.png").write_bytes(png(quarter()))
