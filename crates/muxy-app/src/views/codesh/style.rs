use gpui::{Hsla, Rgba};
use muxy_ui::theme::Theme;

pub(crate) const GUTTER: f32 = 12.0;
pub(crate) const RADIUS: f32 = 10.0;
pub(crate) const HEADER: f32 = 36.0;
// Upstream terminal padding is absorbed by the card body inset.
pub(crate) const BASE_TERMINAL_PADDING: f32 = 2.0;
pub(crate) const BODY_PADDING: f32 = 12.0;
pub(crate) const SIDEBAR_PADDING: f32 = 16.0;
/// Codesh never shows the status bar; the setting stays for upstream code paths.
pub(crate) const STATUS_BAR: bool = false;

fn blend(foreground: Hsla, background: Hsla, amount: f32) -> Hsla {
    let top: Rgba = foreground.into();
    let bottom: Rgba = background.into();
    Rgba {
        r: top.r * amount + bottom.r * (1.0 - amount),
        g: top.g * amount + bottom.g * (1.0 - amount),
        b: top.b * amount + bottom.b * (1.0 - amount),
        a: 1.0,
    }
    .into()
}

pub(crate) fn card(theme: &Theme) -> Hsla {
    blend(theme.fg, theme.bg, 0.025)
}

pub(crate) fn border(theme: &Theme) -> Hsla {
    blend(theme.border, theme.bg, 0.95)
}

/// First extent, gutter extent, second origin. Snap only the shared edge.
pub(crate) fn gutter_geometry(extent: f32, ratio: f32, scale: f32) -> [f32; 3] {
    let extent = extent.max(0.0);
    let gutter = extent.min(GUTTER);
    let available = extent - gutter;
    let first = ((available * ratio * scale).round() / scale).clamp(0.0, available);
    [first, gutter, first + gutter]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gutter_conserves_extent_at_fractional_scales_and_tiny_sizes() {
        for extent in [0.0, 5.0, 12.0, 101.0, 1001.0] {
            for scale in [1.0, 1.5, 2.0] {
                for ratio in [0.15, 0.5, 0.85] {
                    let [first, gutter, second] = gutter_geometry(extent, ratio, scale);
                    assert!(first >= 0.0 && second <= extent);
                    assert!((gutter - extent.min(GUTTER)).abs() < f32::EPSILON);
                    assert!((first + gutter + (extent - second) - extent).abs() < 0.001);
                    if extent > GUTTER {
                        assert!((first * scale - (first * scale).round()).abs() < 0.001);
                    }
                }
            }
        }
    }
}
