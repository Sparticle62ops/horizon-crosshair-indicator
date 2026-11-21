//! crosshair-indicator (Horizon Rust port) on the mod API.
//!
//! Upstream (CayMac97/crosshair-indicator, MIT, (c) CayMac; itself a 1:1 port
//! of AmerDz01's Java Crosshair Indicator) keeps its four settings in the
//! game's native settings storage and draws its HUD overlays (brackets, dot,
//! diamond, chevron, cooldown bar) from a native_hud hook. The mod API wires
//! what the pinned SDK (api 0.2, f00125f61f) reaches:
//!
//! - the `enabled` and `cooldown` toggles register as settings on the
//!   Options > Mods page, same defaults, upstream-style labels;
//! - `client_setup` loads `config/crosshair_indicator.json` (defaults when
//!   missing or broken, like the native settings storage), overlays the two
//!   registered toggles into the live settings cell and stores the enabled
//!   flag, mirroring upstream's `settings()` reader;
//! - the style and theme pickers stay config-file entries because
//!   SettingSpec only takes toggles and ranges.
//!
//! The HUD draw, the key toggle, the native mod-list entry and the settings
//! click cycling are the engine seam: the mod API has no HUD sprite or
//! crosshair-region draw hook, no key hook and no native mod-list hook, so
//! that half is recorded on the seam (tracked in the README and upstream
//! issue) until one lands.
use std::path::PathBuf;
use std::sync::{OnceLock, RwLock};

use mod_api::{ClientSetupEvent, ClientHost, RegistrationContext, Registrar, SettingSpec};

mod runtime;

/// The game directory (upstream: the game's config root).
pub fn game_dir() -> PathBuf {
    std::env::var_os("XI_GAME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

/// `config/crosshair_indicator.json`.
pub fn config_path() -> PathBuf {
    game_dir().join("config").join(runtime::CONFIG_FILE)
}

/// The live settings the HUD seam reads (upstream: the native settings
/// storage read back each frame).
static LIVE_SETTINGS: OnceLock<RwLock<runtime::Settings>> = OnceLock::new();

pub fn set_live_settings(settings: runtime::Settings) {
    let cell = LIVE_SETTINGS.get_or_init(|| RwLock::new(runtime::Settings::default()));
    *cell.write().expect("the settings cell is not poisoned") = settings;
}

pub fn live_settings() -> runtime::Settings {
    LIVE_SETTINGS
        .get()
        .and_then(|cell| cell.read().ok())
        .map(|guard| guard.clone())
        .unwrap_or_default()
}

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl mod_api::Hooks for Mod {
    /// The two toggle settings the API can express, same defaults; the
    /// style/theme enums stay file-held (SettingSpec takes only toggles and
    /// ranges).
    fn register(&self, registrar: &mut dyn Registrar, _context: &RegistrationContext) {
        let _ = registrar.register_setting(SettingSpec::toggle(
            "enabled",
            "setting.crosshair_indicator.enabled",
            true,
        ));
        let _ = registrar.register_setting(SettingSpec::toggle(
            "cooldown",
            "setting.crosshair_indicator.showCooldown",
            true,
        ));
    }

    /// 1:1 of the upstream settings load: read the stored keys (defaults
    /// when missing or broken), overlay the two registered toggles the game
    /// holds, and store the enabled flag the HUD seam checks.
    fn client_setup(&self, host: &mut dyn ClientHost, _event: &mut ClientSetupEvent) {
        let mut settings = std::fs::read_to_string(config_path())
            .ok()
            .map(|text| runtime::Settings::from_json(&text))
            .unwrap_or_default();
        if let Some(value) = host.setting_bool("enabled") {
            settings.enabled = value;
        }
        if let Some(value) = host.setting_bool("cooldown") {
            settings.show_cooldown = value;
        }
        runtime::set_settings(settings.clone());
        set_live_settings(settings);
    }
}
