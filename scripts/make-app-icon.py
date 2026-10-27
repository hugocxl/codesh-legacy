#!/usr/bin/env python3
"""Generate the Codesh app icons.

Writes, relative to the repository root:
  packaging/macos/AppIcon.png      1024x1024 RGBA, stable channel
  packaging/macos/AppIconBeta.png  1024x1024 RGBA, with a "BETA" badge
  packaging/macos/AppIconBeta.svg  vector version of the beta icon

Layout follows upstream's PNGs: a pre-rounded (continuous-corner) tile on the
macOS icon grid, 864x864 at offset 80, transparent outside. The build scripts
(scripts/build-release.sh, scripts/build-install.py) downscale the PNG with
sips into an .iconset, so nothing else needs to change.

Requires Pillow. It is not a project dependency; use a throwaway venv:

    python3 -m venv /tmp/iconenv
    /tmp/iconenv/bin/pip install pillow
    /tmp/iconenv/bin/python scripts/make-app-icon.py [--preview-dir DIR]

--preview-dir additionally writes icon-preview-{16,32,128}.png downscales.
"""

import argparse
import math
from pathlib import Path

from PIL import Image, ImageDraw, ImageFilter, ImageFont

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "packaging" / "macos"

SIZE = 1024
SS = 4  # supersampling factor
TILE_X, TILE_SIZE = 80, 864  # Apple icon grid, same as upstream
SQUIRCLE_N = 4.6  # superellipse exponent approximating continuous corners

BG_TOP = (0x1C, 0x1B, 0x1A)
BG_BOTTOM = (0x12, 0x12, 0x12)
BORDER = (0x2A, 0x2A, 0x2A)
ACCENT = (0xE8, 0x73, 0x4A)
BADGE_FILL = (0xE8, 0x73, 0x4A)
BADGE_TEXT = (0x16, 0x16, 0x16)

# Glyph geometry in 1024 space. ">" is a round-capped polyline, "_" a pill.
STROKE = 100
CHEVRON = [(312, 352), (492, 512), (312, 672)]
UNDERSCORE = (560, 622, 760, 702)  # x0, y0, x1, y1
# Badge (beta only), bottom-right, kept inside the squircle.
BADGE = (600, 770, 872, 862)
BADGE_LABEL = "BETA"


def squircle_points(steps=720):
    c = TILE_X + TILE_SIZE / 2
    r = TILE_SIZE / 2
    pts = []
    for i in range(steps):
        t = 2 * math.pi * i / steps
        ct, st = math.cos(t), math.sin(t)
        x = c + r * math.copysign(abs(ct) ** (2 / SQUIRCLE_N), ct)
        y = c + r * math.copysign(abs(st) ** (2 / SQUIRCLE_N), st)
        pts.append((x, y))
    return pts


def scaled(pts, k=SS):
    return [(x * k, y * k) for x, y in pts]


def draw_glyph(draw, k, fill):
    w = STROKE * k
    pts = scaled(CHEVRON, k)
    # Segments drawn separately with round caps (Pillow's joint="curve" leaves
    # a notch at the apex).
    for a, b in zip(pts, pts[1:]):
        draw.line((a, b), fill=fill, width=w)
    for x, y in pts:
        draw.ellipse((x - w / 2, y - w / 2, x + w / 2, y + w / 2), fill=fill)
    x0, y0, x1, y1 = (v * k for v in UNDERSCORE)
    draw.rounded_rectangle((x0, y0, x1, y1), radius=(y1 - y0) / 2, fill=fill)


def load_bold_font(px):
    for path, index, variation in (
        ("/System/Library/Fonts/SFNSRounded.ttf", 0, "Bold"),
        ("/System/Library/Fonts/SFNS.ttf", 0, "Bold"),
        ("/System/Library/Fonts/Helvetica.ttc", 1, None),
    ):
        try:
            font = ImageFont.truetype(path, px, index=index)
            if variation:
                try:
                    font.set_variation_by_name(variation)
                except Exception:
                    pass
            return font
        except OSError:
            continue
    return ImageFont.load_default(px)


def render(beta):
    S = SIZE * SS
    tile = scaled(squircle_points())

    mask = Image.new("L", (S, S), 0)
    ImageDraw.Draw(mask).polygon(tile, fill=255)

    # Vertical gradient background.
    grad = Image.new("RGB", (1, S))
    y0, y1 = TILE_X * SS, (TILE_X + TILE_SIZE) * SS
    for y in range(S):
        t = min(max((y - y0) / (y1 - y0), 0), 1)
        grad.putpixel((0, y), tuple(round(a + (b - a) * t) for a, b in zip(BG_TOP, BG_BOTTOM)))
    bg = grad.resize((S, S)).convert("RGBA")

    img = Image.new("RGBA", (S, S), (0, 0, 0, 0))
    img.paste(bg, (0, 0), mask)

    # Hairline border (~2px at 1024) just inside the tile edge.
    inner = mask.filter(ImageFilter.MinFilter(2 * SS + 1))
    ring = Image.eval(Image.composite(Image.new("L", (S, S), 0), mask, inner), lambda v: v)
    img.paste(Image.new("RGBA", (S, S), BORDER + (255,)), (0, 0), ring)

    # Soft glow behind the glyph, then the glyph itself.
    glow = Image.new("L", (S, S), 0)
    draw_glyph(ImageDraw.Draw(glow), SS, 255)
    glow = glow.filter(ImageFilter.GaussianBlur(36 * SS)).point(lambda v: v * 0.45)
    glow_layer = Image.new("RGBA", (S, S), ACCENT + (255,))
    img.paste(glow_layer, (0, 0), Image.composite(glow, Image.new("L", (S, S), 0), mask))
    draw_glyph(ImageDraw.Draw(img), SS, ACCENT + (255,))

    if beta:
        d = ImageDraw.Draw(img)
        x0, y0, x1, y1 = (v * SS for v in BADGE)
        h = y1 - y0
        d.rounded_rectangle((x0, y0, x1, y1), radius=h / 2, fill=BADGE_FILL + (255,))
        font = load_bold_font(int(h * 0.62))
        bx = d.textbbox((0, 0), BADGE_LABEL, font=font)
        tw, th = bx[2] - bx[0], bx[3] - bx[1]
        d.text(((x0 + x1 - tw) / 2 - bx[0], (y0 + y1 - th) / 2 - bx[1]),
               BADGE_LABEL, font=font, fill=BADGE_TEXT + (255,))

    return img.resize((SIZE, SIZE), Image.LANCZOS)


def svg(beta):
    hexc = lambda c: "#%02x%02x%02x" % c
    path = "M" + " L".join(f"{x:.1f},{y:.1f}" for x, y in squircle_points(360)) + " Z"
    chev = " ".join(f"{x},{y}" for x, y in CHEVRON)
    ux0, uy0, ux1, uy1 = UNDERSCORE
    glyph = (
        f'<polyline points="{chev}" fill="none" stroke="{hexc(ACCENT)}" stroke-width="{STROKE}" '
        f'stroke-linecap="round" stroke-linejoin="round"/>'
        f'<rect x="{ux0}" y="{uy0}" width="{ux1 - ux0}" height="{uy1 - uy0}" rx="{(uy1 - uy0) / 2}" '
        f'fill="{hexc(ACCENT)}"/>'
    )
    parts = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{SIZE}" height="{SIZE}" viewBox="0 0 {SIZE} {SIZE}">',
        "<defs>",
        f'<linearGradient id="bg" x1="0" y1="0" x2="0" y2="1">'
        f'<stop offset="0" stop-color="{hexc(BG_TOP)}"/><stop offset="1" stop-color="{hexc(BG_BOTTOM)}"/>'
        "</linearGradient>",
        '<filter id="glow" x="-50%" y="-50%" width="200%" height="200%">'
        '<feGaussianBlur stdDeviation="36"/></filter>',
        f'<clipPath id="tile"><path d="{path}"/></clipPath>',
        "</defs>",
        f'<path d="{path}" fill="url(#bg)" stroke="{hexc(BORDER)}" stroke-width="4" clip-path="url(#tile)"/>',
        f'<g clip-path="url(#tile)" opacity="0.45" filter="url(#glow)">{glyph}</g>',
        glyph,
    ]
    if beta:
        x0, y0, x1, y1 = BADGE
        h = y1 - y0
        parts.append(
            f'<rect x="{x0}" y="{y0}" width="{x1 - x0}" height="{h}" rx="{h / 2}" fill="{hexc(BADGE_FILL)}"/>'
            f'<text x="{(x0 + x1) / 2}" y="{(y0 + y1) / 2}" text-anchor="middle" dominant-baseline="central" '
            f'font-family="SF Pro Rounded, -apple-system, Helvetica, Arial, sans-serif" font-weight="700" '
            f'font-size="{round(h * 0.62)}" fill="{hexc(BADGE_TEXT)}">{BADGE_LABEL}</text>'
        )
    parts.append("</svg>\n")
    return "\n".join(parts)


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--preview-dir", type=Path)
    args = ap.parse_args()

    stable = render(beta=False)
    beta = render(beta=True)
    stable.save(OUT / "AppIcon.png", optimize=True)
    beta.save(OUT / "AppIconBeta.png", optimize=True)
    (OUT / "AppIconBeta.svg").write_text(svg(beta=True))

    if args.preview_dir:
        args.preview_dir.mkdir(parents=True, exist_ok=True)
        for px in (16, 32, 128):
            stable.resize((px, px), Image.LANCZOS).save(args.preview_dir / f"icon-preview-{px}.png")
        for px in (32, 128):
            beta.resize((px, px), Image.LANCZOS).save(args.preview_dir / f"icon-preview-beta-{px}.png")


if __name__ == "__main__":
    main()
