//! Crosshair Indicator 1.0.1 on the pinned SDK (api 0.2, f00125f61f):
//! CayMac97's Horizon mod (`github.com/CayMac97/crosshair-indicator`, MIT)
//! brought to the current mod API. MIT carried with the upstream credit.
//!
//! - `runtime` carries the upstream runtime 1:1 (styles, themes, settings,
//!   the ENABLED atomic);
//! - `glue` carries the upstream glue: the exact HUD fill plan and the
//!   toggle logic;
//! - `seams` tracks the engine-stage gaps (no crosshair render hook).
//!
//! Mod API wiring:
//! - the four settings register as settings (`enabled` toggle default on,
//!   `style` choice of 4, `theme` choice of 6, `cooldown` toggle default
//!   on), upstream labels;
//! - the `toggle` key registers as a key mapping (unbound default, as
//!   upstream's `KEY_TOGGLE_DEFAULT`);
//! - `client_setup` syncs the runtime model from the host's setting reads;
//! - `setting_changed` re-syncs it when the player changes a setting;
//! - `client_tick_post` consumes the toggle mapping's click and flips the
//!   setting through the host.

use mod_api::{
    ClientHost, ClientSetupEvent, ClientTickPostEvent, KeyConflictContext, KeyRegistration,
    RegistrationContext, Registrar, SettingChangedEvent, SettingSpec,
};

pub mod glue;
pub mod runtime;
pub mod seams;
#[cfg(test)]
pub mod tests;

pub const MOD_ID: &str = runtime::ID;

#[derive(Default)]
pub struct Mod;

#[mod_api::horizon_mod]
impl mod_api::Hooks for Mod {
    /// The four settings and the toggle key, upstream labels.
    fn register(&self, registrar: &mut dyn Registrar, _context: &RegistrationContext) {
        let _ = registrar.register_setting(SettingSpec::toggle(
            "enabled",
            "Crosshair Indicator: ON",
            true,
        ));
        let _ = registrar.register_setting(SettingSpec::choice(
            "style",
            "Style: Brackets [ + ]",
            [
                "Brackets [ + ]",
                "Center Dot",
                "Diamond",
                "Chevron",
            ],
            0,
        ));
        let _ = registrar.register_setting(SettingSpec::choice(
            "theme",
            "Color: Classic White (Default)",
            [
                "Classic White (Default)",
                "Dynamic (Green/Orange)",
                "Combat Red",
                "Pink Aesthetic",
                "Diamond Cyan",
                "Gold",
            ],
            0,
        ));
        let _ = registrar.register_setting(SettingSpec::toggle(
            "cooldown",
            "Cooldown Bar: ON",
            true,
        ));
        let _ = registrar.register_key_mapping(
            KeyRegistration::new("toggle", "key.keyboard.unknown")
                .label("Toggle Crosshair Indicator"),
        );
    }

    /// java/glue: the initial model sync from the host's setting reads.
    fn client_setup(&self, host: &mut dyn ClientHost, _event: &mut ClientSetupEvent) {
        sync_from_host(host);
    }

    /// java/glue: on_key's toggle through the tick's consume_click.
    fn client_tick_post(&self, host: &mut dyn ClientHost, _event: &mut ClientTickPostEvent) {
        if host.consume_click(MOD_ID, "toggle") {
            let s = runtime::settings(|k| {
                match k {
                    "enabled" => host.setting_bool("enabled").map(|b| if b { "1" } else { "0" }.to_string()),
                    "style" => host.setting_choice("style").map(|v| v.to_string()),
                    "theme" => host.setting_choice("theme").map(|v| v.to_string()),
                    "cooldown" => host.setting_bool("cooldown").map(|b| if b { "1" } else { "0" }.to_string()),
                    _ => None,
                }
            });
            if let Some(next) = glue::toggle("toggle", true, s) {
                persist(host, &next);
            }
        }
        generated_clear();
    }

    /// The player changed a setting on the page; re-sync the model.
    fn setting_changed(&self, host: &mut dyn ClientHost, _event: &mut SettingChangedEvent<'_>) {
        sync_from_host(host);
    }
}

/// java/glue: `m::settings(read)` against the host's typed reads.
fn sync_from_host(host: &dyn ClientHost) {
    let s = runtime::settings(|k| {
        match k {
            "enabled" => host.setting_bool("enabled").map(|b| if b { "1" } else { "0" }.to_string()),
            "style" => host.setting_choice("style").map(|v| v.to_string()),
            "theme" => host.setting_choice("theme").map(|v| v.to_string()),
            "cooldown" => host.setting_bool("cooldown").map(|b| if b { "1" } else { "0" }.to_string()),
            _ => None,
        }
    });
    runtime::set_settings(s);
}

/// java/glue: the settings write-back (`native_set_setting`).
fn persist(host: &dyn ClientHost, s: &runtime::Settings) {
    for (k, v) in s.stored() {
        host.set_setting(MOD_ID, k, &v);
    }
    runtime::set_settings(s.clone());
}

fn generated_clear() {}
