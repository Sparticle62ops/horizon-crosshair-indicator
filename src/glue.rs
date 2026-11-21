//! The glue between the mod API and the runtime model, java-equivalent of
//! CayMac's `mixins/crosshair_indicator/glue.rs` (MIT): the HUD draw plan,
//! the toggle key, the mod list entry and the settings buttons. The draw
//! itself runs behind the render seam (`seams.rs`); everything else carries
//! the upstream logic verbatim.

use crate::runtime::{self, Settings};
use crate::runtime as m;

/// The fill list the HUD draw would carry, computed with the exact
/// coordinates of the upstream `on_hud` (the 15x15 vanilla crosshair centred
/// at `floor((w - 15) / 2)`, center pixel at +7.0). `strength` is the attack
/// strength scaled to 0.0..=speed, `target_in_reach` the crosshair living
/// check. Colors follow the theme's `resolve(charged)`.
pub fn render_plan(
    wi: f64,
    hi: f64,
    strength: f64,
    target_in_reach: bool,
    s: &Settings,
) -> Vec<([f64; 4], u32)> {
    let mut fills = Vec::new();
    if wi <= 0.0 || hi <= 0.0 || !s.enabled {
        return fills;
    }
    let speed = 1.0; // the host-side attack speed input (seam: world reads)
    let strength = strength.clamp(0.0, speed) / speed;
    let charged = strength >= 0.98;
    let cx = ((wi - 15.0) / 2.0).floor() + 7.0;
    let cy = ((hi - 15.0) / 2.0).floor() + 7.0;
    let color = s.theme.resolve(charged);
    if target_in_reach {
        match s.style {
            m::Style::Brackets => {
                // Top-Left corner:
                fills.push(([cx - 3.0, cy - 3.0, 2.0, 1.0], color));
                fills.push(([cx - 3.0, cy - 2.0, 1.0, 1.0], color));
                // Top-Right corner:
                fills.push(([cx + 2.0, cy - 3.0, 2.0, 1.0], color));
                fills.push(([cx + 3.0, cy - 2.0, 1.0, 1.0], color));
                // Bottom-Left corner:
                fills.push(([cx - 3.0, cy + 2.0, 1.0, 1.0], color));
                fills.push(([cx - 3.0, cy + 3.0, 2.0, 1.0], color));
                // Bottom-Right corner:
                fills.push(([cx + 3.0, cy + 2.0, 1.0, 1.0], color));
                fills.push(([cx + 2.0, cy + 3.0, 2.0, 1.0], color));
            }
            m::Style::CenterDot => fills.push(([cx, cy, 1.0, 1.0], color)),
            m::Style::Diamond => {
                fills.push(([cx, cy - 5.0, 1.0, 1.0], color));
                fills.push(([cx, cy + 5.0, 1.0, 1.0], color));
                fills.push(([cx - 5.0, cy, 1.0, 1.0], color));
                fills.push(([cx + 5.0, cy, 1.0, 1.0], color));
            }
            m::Style::Chevron => {
                fills.push(([cx - 2.0, cy - 6.0, 5.0, 1.0], color));
                fills.push(([cx - 1.0, cy - 5.0, 3.0, 1.0], color));
                fills.push(([cx, cy - 4.0, 1.0, 1.0], color));
            }
        }
    }
    // 2. Optional mini cooldown bar below crosshair when weapon is charging
    if s.show_cooldown && !charged {
        let bar_w = 16.0;
        let bar_h = 2.0;
        let bar_x = (cx - 8.0).floor();
        let bar_y = cy + 10.0;
        fills.push(([bar_x, bar_y, bar_w, bar_h], runtime::COLOR_GRAY));
        let fill_w = (bar_w * strength.clamp(0.0, 1.0)).round();
        if fill_w > 0.0 {
            fills.push(([bar_x, bar_y, fill_w, bar_h], runtime::COLOR_WHITE));
        }
    }
    fills
}

/// java/glue: on_key's toggle, minus the `native_on`/`native_playing` gates
/// the registration covers. Returns the settings to persist.
pub fn toggle(key: &str, pressed: bool, s: Settings) -> Option<Settings> {
    if !pressed {
        return None;
    }
    if key == m::KEY_TOGGLE_DEFAULT && m::KEY_TOGGLE_DEFAULT != "key.keyboard.unknown" {
        return None;
    }
    let mut s = s;
    s.enabled = !s.enabled;
    Some(s)
}
