//! Crosshair Indicator runtime module.
//!
//! Exact 1:1 Rust port of upstream `mixins/crosshair_indicator/runtime.rs`
//! (CayMac97/crosshair-indicator, MIT, (c) CayMac), itself a 1:1 port of
//! AmerDz01's Java Crosshair Indicator. The game-side glue (`on_hud`,
//! `on_key`, `on_list`, `on_settings`, `on_click`) rides the engine seam:
//! the mod API has no HUD sprite/crosshair-region draw hook, no key hook and
//! no native mod-list hook, so the render work is recorded on the seam and
//! the settings reach the Options > Mods page through the mod API instead of
//! the upstream native-settings storage.
use std::sync::atomic::{AtomicBool, Ordering};

pub const ID: &str = "crosshair_indicator";
pub const NAME: &str = "Crosshair Indicator";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Dynamic attack reach awareness: visual feedback on your crosshair when hovering over an entity in range.";

pub const KEY_TOGGLE: &str = "toggle";
pub const KEY_TOGGLE_LABEL: &str = "Toggle Crosshair Indicator";
pub const KEY_TOGGLE_DEFAULT: &str = "key.keyboard.unknown";

// Color constants (0xAARRGGBB)
pub const COLOR_WHITE: u32 = 0xFFFFFFFF; // Classic PvP White
pub const COLOR_GREEN: u32 = 0xFF00FF7F; // Spring Green (ready)
pub const COLOR_ORANGE: u32 = 0xFFFF8C00; // Dark Orange (charging)
pub const COLOR_RED: u32 = 0xFFFF3333; // Vivid Red
pub const COLOR_PINK: u32 = 0xFFFF69B4; // Hot Pink
pub const COLOR_CYAN: u32 = 0xFF00FFFF; // Diamond Cyan
pub const COLOR_GOLD: u32 = 0xFFFFD700; // Rich Gold
pub const COLOR_GRAY: u32 = 0x80202020; // Cooldown background

/// The config file the port keeps the settings the settings API cannot
/// express (the style and theme enums; the two toggles register as settings
/// and overlay it on client setup). Keys are exactly upstream `stored()`.
pub const CONFIG_FILE: &str = "crosshair_indicator.json";

static ENABLED: AtomicBool = AtomicBool::new(true);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Style {
    Brackets,
    CenterDot,
    Diamond,
    Chevron,
}

impl Style {
    pub fn from_index(i: usize) -> Self {
        match i % 4 {
            0 => Self::Brackets,
            1 => Self::CenterDot,
            2 => Self::Diamond,
            _ => Self::Chevron,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Brackets => "Brackets [ + ]",
            Self::CenterDot => "Center Dot",
            Self::Diamond => "Diamond",
            Self::Chevron => "Chevron",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorTheme {
    White,
    Dynamic,
    Red,
    Pink,
    Cyan,
    Gold,
}

impl ColorTheme {
    pub fn from_index(i: usize) -> Self {
        match i % 6 {
            0 => Self::White,
            1 => Self::Dynamic,
            2 => Self::Red,
            3 => Self::Pink,
            4 => Self::Cyan,
            _ => Self::Gold,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::White => "Classic White (Default)",
            Self::Dynamic => "Dynamic (Green/Orange)",
            Self::Red => "Combat Red",
            Self::Pink => "Pink Aesthetic",
            Self::Cyan => "Diamond Cyan",
            Self::Gold => "Gold",
        }
    }

    pub fn resolve(self, charged: bool) -> u32 {
        match self {
            Self::White => COLOR_WHITE,
            Self::Dynamic => {
                if charged {
                    COLOR_GREEN
                } else {
                    COLOR_ORANGE
                }
            }
            Self::Red => COLOR_RED,
            Self::Pink => COLOR_PINK,
            Self::Cyan => COLOR_CYAN,
            Self::Gold => COLOR_GOLD,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    pub enabled: bool,
    pub style: Style,
    pub theme: ColorTheme,
    pub show_cooldown: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            enabled: true,
            style: Style::Brackets,
            theme: ColorTheme::White,
            show_cooldown: true,
        }
    }
}

impl Settings {
    pub fn labels(&self) -> Vec<String> {
        vec![
            format!(
                "Crosshair Indicator: {}",
                if self.enabled { "ON" } else { "OFF" }
            ),
            format!("Style: {}", self.style.name()),
            format!("Color: {}", self.theme.name()),
            format!(
                "Cooldown Bar: {}",
                if self.show_cooldown { "ON" } else { "OFF" }
            ),
        ]
    }

    pub fn click(&mut self, button: usize) {
        match button {
            0 => self.enabled = !self.enabled,
            1 => self.style = Style::from_index((self.style as usize) + 1),
            2 => self.theme = ColorTheme::from_index((self.theme as usize) + 1),
            3 => self.show_cooldown = !self.show_cooldown,
            _ => {}
        }
        ENABLED.store(self.enabled, Ordering::Relaxed);
    }

    pub fn stored(&self) -> Vec<(&'static str, String)> {
        vec![
            ("enabled", if self.enabled { "1" } else { "0" }.into()),
            ("style", (self.style as usize).to_string()),
            ("theme", (self.theme as usize).to_string()),
            (
                "cooldown",
                if self.show_cooldown { "1" } else { "0" }.into(),
            ),
        ]
    }

    /// JSON round-trip of the stored keys (upstream keeps them in the game's
    /// native settings storage; the port file mirrors those key names).
    pub fn from_json(text: &str) -> Settings {
        let mut s = Settings::default();
        let value: serde_json::Value = match serde_json::from_str(text) {
            Ok(value) => value,
            Err(_) => return s,
        };
        if let Some(v) = value.get("enabled").and_then(|v| v.as_str()) {
            s.enabled = v == "1" || v.eq_ignore_ascii_case("true");
        }
        if let Some(v) = value
            .get("style")
            .and_then(|v| v.as_str())
            .and_then(|v| v.parse::<usize>().ok())
        {
            s.style = Style::from_index(v);
        }
        if let Some(v) = value
            .get("theme")
            .and_then(|v| v.as_str())
            .and_then(|v| v.parse::<usize>().ok())
        {
            s.theme = ColorTheme::from_index(v);
        }
        if let Some(v) = value.get("cooldown").and_then(|v| v.as_str()) {
            s.show_cooldown = v == "1" || v.eq_ignore_ascii_case("true");
        }
        s
    }

    pub fn to_json(&self) -> String {
        let mut pairs: Vec<String> = Vec::new();
        for (key, value) in self.stored() {
            pairs.push(format!("\"{}\": \"{}\"", key, value));
        }
        format!("{{{}}}", pairs.join(", "))
    }
}

pub fn settings(mut read: impl FnMut(&str) -> Option<String>) -> Settings {
    let mut s = Settings::default();
    if let Some(v) = read("enabled") {
        s.enabled = v == "1" || v.eq_ignore_ascii_case("true");
    }
    if let Some(v) = read("style").and_then(|v| v.parse::<usize>().ok()) {
        s.style = Style::from_index(v);
    }
    if let Some(v) = read("theme").and_then(|v| v.parse::<usize>().ok()) {
        s.theme = ColorTheme::from_index(v);
    }
    if let Some(v) = read("cooldown") {
        s.show_cooldown = v == "1" || v.eq_ignore_ascii_case("true");
    }
    ENABLED.store(s.enabled, Ordering::Relaxed);
    s
}

pub fn is_enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

pub fn set_settings(s: Settings) {
    ENABLED.store(s.enabled, Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn style_cycle_matches_upstream() {
        assert_eq!(Style::from_index(0), Style::Brackets);
        assert_eq!(Style::from_index(1), Style::CenterDot);
        assert_eq!(Style::from_index(2), Style::Diamond);
        assert_eq!(Style::from_index(3), Style::Chevron);
        assert_eq!(Style::from_index(4), Style::Brackets);
    }

    #[test]
    fn theme_cycle_matches_upstream() {
        assert_eq!(ColorTheme::from_index(0), ColorTheme::White);
        assert_eq!(ColorTheme::from_index(5), ColorTheme::Gold);
        assert_eq!(ColorTheme::from_index(6), ColorTheme::White);
    }

    #[test]
    fn theme_resolves_charge_colors() {
        assert_eq!(ColorTheme::Dynamic.resolve(true), COLOR_GREEN);
        assert_eq!(ColorTheme::Dynamic.resolve(false), COLOR_ORANGE);
        assert_eq!(ColorTheme::White.resolve(false), COLOR_WHITE);
        assert_eq!(ColorTheme::Gold.resolve(true), COLOR_GOLD);
    }

    #[test]
    fn defaults_match_upstream() {
        let s = Settings::default();
        assert!(s.enabled);
        assert_eq!(s.style, Style::Brackets);
        assert_eq!(s.theme, ColorTheme::White);
        assert!(s.show_cooldown);
    }

    #[test]
    fn labels_list_all_four_lines() {
        let s = Settings::default();
        let labels = s.labels();
        assert_eq!(
            labels,
            vec![
                "Crosshair Indicator: ON".to_string(),
                "Style: Brackets [ + ]".to_string(),
                "Color: Classic White (Default)".to_string(),
                "Cooldown Bar: ON".to_string(),
            ]
        );
    }

    #[test]
    fn click_cycles_each_row_and_syncs_enabled() {
        let mut s = Settings::default();
        s.click(0);
        assert!(!s.enabled);
        assert!(!is_enabled(), "click stores the enabled state");
        s.click(1);
        assert_eq!(s.style, Style::CenterDot);
        s.click(2);
        assert_eq!(s.theme, ColorTheme::Dynamic);
        s.click(3);
        assert!(!s.show_cooldown);
        s.click(4); // out of range leaves everything as-is
        assert!(!s.enabled && !s.show_cooldown);
    }

    #[test]
    fn stored_writes_upstream_key_names() {
        let s = Settings::default();
        assert_eq!(
            s.stored(),
            vec![
                ("enabled", "1".to_string()),
                ("style", "0".to_string()),
                ("theme", "0".to_string()),
                ("cooldown", "1".to_string()),
            ]
        );
    }

    #[test]
    fn json_round_trip_keeps_stored_keys() {
        let s = Settings::default();
        let text = s.to_json();
        assert_eq!(Settings::from_json(&text), s);
    }

    #[test]
    fn json_reader_overrides_and_falls_back() {
        let text = r#"{"enabled": "0", "style": "5", "theme": "x", "cooldown": "true"}"#;
        let s = Settings::from_json(text);
        assert!(!s.enabled);
        assert_eq!(s.style, Style::CenterDot, "style index wraps mod 4 (5 % 4 = 1)");
        assert_eq!(s.theme, ColorTheme::White, "an unparsable theme falls back");
        assert!(s.show_cooldown, "true parses like 1");
    }

    #[test]
    fn broken_json_leaves_defaults() {
        let s = Settings::from_json("{not json");
        assert_eq!(s, Settings::default());
    }

    #[test]
    fn settings_reader_1_to_1() {
        let map = [
            ("enabled", "1"),
            ("style", "3"),
            ("theme", "2"),
            ("cooldown", "0"),
        ];
        let s = settings(|k| map.iter().find(|(key, _)| *key == k).map(|(_, v)| v.to_string()));
        assert!(s.enabled);
        assert_eq!(s.style, Style::Chevron);
        assert_eq!(s.theme, ColorTheme::Red);
        assert!(!s.show_cooldown);
        assert!(is_enabled());
        let missing = settings(|_| None);
        assert_eq!(missing, Settings::default());
    }

    #[test]
    fn set_settings_stores_enabled() {
        let mut s = Settings::default();
        s.enabled = false;
        set_settings(s.clone());
        assert!(!is_enabled());
        s.enabled = true;
        set_settings(s);
        assert!(is_enabled());
    }
}
