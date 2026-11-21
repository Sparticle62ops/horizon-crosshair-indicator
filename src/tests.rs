//! Tests over the carried model and the render plan.

use crate::glue::render_plan;
use crate::runtime as m;
use crate::runtime::{ColorTheme, Settings, Style};

#[test]
fn style_and_theme_wrap() {
    assert_eq!(Style::from_index(0), Style::Brackets);
    assert_eq!(Style::from_index(4), Style::Brackets);
    assert_eq!(Style::from_index(5), Style::CenterDot);
    assert_eq!(ColorTheme::from_index(6), ColorTheme::White);
    assert_eq!(ColorTheme::from_index(11), ColorTheme::Gold);
}

#[test]
fn theme_resolve_colors() {
    assert_eq!(ColorTheme::White.resolve(true), m::COLOR_WHITE);
    assert_eq!(ColorTheme::Dynamic.resolve(true), m::COLOR_GREEN);
    assert_eq!(ColorTheme::Dynamic.resolve(false), m::COLOR_ORANGE);
    assert_eq!(ColorTheme::Red.resolve(false), m::COLOR_RED);
    assert_eq!(ColorTheme::Pink.resolve(false), m::COLOR_PINK);
    assert_eq!(ColorTheme::Cyan.resolve(false), m::COLOR_CYAN);
    assert_eq!(ColorTheme::Gold.resolve(false), m::COLOR_GOLD);
}

#[test]
fn defaults_and_labels() {
    let s = Settings::default();
    assert!(s.enabled && s.show_cooldown);
    assert_eq!(s.style, Style::Brackets);
    assert_eq!(s.theme, ColorTheme::White);
    let labels = s.labels();
    assert_eq!(labels.len(), 4);
    assert!(labels[0].contains("ON"));
    assert!(labels[1].contains("Brackets"));
    assert!(labels[3].contains("ON"));
}

#[test]
fn click_cycles_and_stores() {
    let mut s = Settings::default();
    s.click(1);
    assert_eq!(s.style, Style::CenterDot);
    s.click(2);
    assert_eq!(s.theme, ColorTheme::Dynamic);
    s.click(0);
    assert!(!s.enabled);
    s.click(3);
    assert!(!s.show_cooldown);
    let stored = s.stored();
    assert_eq!(stored[0].1, "0");
    assert_eq!(stored[1].1, "1");
    assert_eq!(stored[2].1, "1");
    assert_eq!(stored[3].1, "0");
    assert!(!m::is_enabled());
}

#[test]
fn settings_read_overrides() {
    let mut read_style: Option<String> = None;
    let s = {
        let read = |k: &str| -> Option<String> {
            match k {
                "enabled" => Some("0".into()),
                "style" => read_style.clone(),
                "theme" => Some("4".into()),
                "cooldown" => Some("0".into()),
                _ => None,
            }
        };
        m::settings(read)
    };
    assert!(!s.enabled);
    assert_eq!(s.theme, ColorTheme::Cyan);
    assert!(!s.show_cooldown);
    assert_eq!(s.style, Style::Brackets);
}

#[test]
fn brackets_plan_matches_upstream_geometry() {
    // 1080x720: cx = floor((1080-15)/2)+7 = 539, cy = floor((720-15)/2)+7 = 359
    let s = Settings::default();
    let plan = render_plan(1080.0, 720.0, 1.0, true, &s);
    // 8 bracket fills
    assert_eq!(plan.len(), 8);
    assert_eq!(plan[0], ([539.0 - 3.0, 359.0 - 3.0, 2.0, 1.0], m::COLOR_WHITE));
    assert_eq!(plan[3], ([539.0 + 3.0, 359.0 - 2.0, 1.0, 1.0], m::COLOR_WHITE));
}

#[test]
fn cooldown_bar_plan() {
    let s = Settings::default();
    // Half-charged: no target fills, gray bar + white fill of 8px
    let plan = render_plan(1080.0, 720.0, 0.5, false, &s);
    assert_eq!(plan.len(), 2);
    assert_eq!(plan[0], ([531.0, 369.0, 16.0, 2.0], m::COLOR_GRAY));
    assert_eq!(plan[1], ([531.0, 369.0, 8.0, 2.0], m::COLOR_WHITE));
}

#[test]
fn disabled_and_gates() {
    let mut s = Settings::default();
    s.enabled = false;
    assert!(render_plan(1080.0, 720.0, 1.0, true, &s).is_empty());
    assert!(render_plan(0.0, 720.0, 1.0, true, &Settings::default()).is_empty());
}

#[test]
fn toggle_flips_enabled() {
    let s = Settings::default();
    let next = crate::glue::toggle("toggle", true, s).unwrap();
    assert!(!next.enabled);
    assert!(crate::glue::toggle("toggle", false, Settings::default()).is_none());
}
