# Crosshair Indicator (`crosshair_indicator_port`)

A Horizon client mod: your crosshair tells you when the mob or player you aim
at is within attack reach. Exact 1:1 Rust port of
[CayMac97/crosshair-indicator](https://github.com/CayMac97/crosshair-indicator)
(MIT, CayMac), itself a 1:1 port of AmerDz01's Java Crosshair Indicator.
Carries the upstream license and credit. Port by sparticle.

- Four indicator styles: corner brackets around the crosshair (the default),
  a center dot, diamond pips, or a chevron above it.
- Color themes follow the weapon's charge state: classic white, dynamic
  (green when fully charged, orange while charging), combat red, pink,
  diamond cyan and gold.
- Optional thin weapon-cooldown bar below the crosshair.

## What the mod API wires (SDK f00125f, mixins e59da909, api 0.2)

- `enabled` and `cooldown` (cooldown bar) register as toggles on the
  Options > Mods page, same defaults, upstream-style labels.
- `client_setup` loads `config/crosshair_indicator.json` (defaults when
  missing or broken, mirroring upstream's native-settings storage), overlays
  the two registered toggles, and stores the enabled flag.
- The style and theme pickers stay config-file entries: SettingSpec only
  takes toggles and ranges. Keys match upstream's stored names (`enabled`,
  `style`, `theme`, `cooldown`).

## Engine seams (tracked)

The mod API has no HUD sprite or crosshair-region draw hook, no key hook and
no native mod-list hook, so the draw half (brackets/dot/diamond/chevron
fills, the cooldown bar, first-person/spectator/debug-crosshair gating, the
attack-strength and living-target reads), the `toggle` keybind and the
in-game settings click cycling are recorded on the seam until the surfaces
land. Upstream's `runtime.rs` module is ported 1:1 and fully tested; the
`glue.rs` hooks stay unported until the API exists.

## Tests

`cargo test` (12 tests) covers the upstream behavior: style/theme cycling,
theme color resolution, defaults, labels, click cycling with the enabled
sync, the stored key names, the JSON round trip and fallbacks, and the
1:1 `settings()` reader.

## License

MIT (LICENSE carries the upstream text).
