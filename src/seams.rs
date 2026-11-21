//! Tracked seams, engine-stage: the pinned mod API (f00125f / e59da909,
//! api 0.2) has no counterpart for these yet.
//!
//! - No crosshair/HUD render hook: the mod API has no draw surface for the
//!   vanilla crosshair, so the exact fill plan `glue::render_plan` computes
//!   (coordinates, colors and the cooldown bar, all 1:1 of upstream
//!   `on_hud`) is not painted. The reviewer's follow-up on the 1.0.0
//!   package noted the same gap. The plan ships tested and complete, ready
//!   for the hook when it lands.
//! - The HUD-side world inputs (first-person check, spectator, debug
//!   crosshair, `gui_size`, `main_hand_attack_speed`, `hand_strength`
//!   scaling, `crosshair_living`) have no host wires; `render_plan` takes
//!   the strength and target as parameters for when they land.
//! - The old `native_mods` list entry and the `native_settings` button rows
//!   map onto the mod API's metadata and settings page: the four settings
//!   register as real settings (toggle/choice) instead of button strings.
//! - `native_key`'s toggle runs on the tick's `consume_click` of the
//!   registered `toggle` mapping: one tick after the press, not on the
//!   press itself.
