# Farmer's Delight — a Horizon port

**Farmer's Delight** is a Minecraft mod. A cozy expansion to farming and cooking in Minecraft. This repository is a **port of it to [Horizon](https://github.com/horizon-rust/horizon)**, the Rust engine for Minecraft Java 26.3. It carries the complete source of the port plus the content files it ships, and installs as a single `.hmod` package.

Upstream: [vectorwing/FarmersDelight](https://github.com/vectorwing/FarmersDelight) — licence **MIT**.

| | |
|---|---|
| Upstream mod | Farmer's Delight ([source](https://github.com/vectorwing/FarmersDelight)) |
| Ported to | Horizon 0b14f8a, Minecraft Java 26.3, mod API `0.2` |
| Package | `farmersdelight.hmod` (install it with the Horizon launcher) |
| Licence | **MIT** — the upstream mod's licence; see [Licence and credit](#licence-and-credit) |

## What this port implements

### What works

- **77 foods** with their nutrition, absolute saturation, stack size, eating time
  and bowl/glass-bottle remainder: raw crops, foodstuffs, sweets, handheld meals,
  soups and stews, plated meals, feast portions, dog food and apple cider.
- The **vanilla effects** the foods apply when eaten (hunger, speed, regeneration,
  nausea, glowing, absorption).
- **Two crops** — `farmersdelight:cabbages` and `farmersdelight:budding_tomatoes` —
  grown by the vanilla crop random tick on hydrated farmland, with bone meal; a
  seed used on the top of farmland plants its crop and spends one seed. A ripe
  cabbage drops a cabbage and a seed, a young one a seed only.
- **45 recipes** (crafting, smelting, smoking, campfire), the `c:` item tags they
  need, and one creative tab listing all 79 items.

## Not yet ported

- **Crop blocks and seeds**: cabbage/tomato/onion crops, wild crops, rice and
- **Cutting board, cooking pot, skillet and their recipe types**: all
- **Feast blocks** (roast chicken block, stuffed pumpkin block, …) and the
- **`nourishment`**: the custom effect is not registered (see above).
- **Non-food drinks**: milk bottle, hot cocoa and melon juice have no
- **Interactions**: dog food feeding wolves and horse feed are deferred (they
- **Compost values, recipe advancements, tooltips and client effect tooltips**
- **Not exercised in-game**: the vanilla consume effects, the bowl/bottle

## Installing

1. Install the [Horizon launcher](https://github.com/horizon-rust/horizon/releases) (0.2.70 or newer).
2. Add `farmersdelight.hmod` to the instance's mods.
3. The package is pinned to a specific Horizon build; the launcher checks that pin.

## Licence and credit

Farmer's Delight was created by its upstream authors; the code, models and assets in the original mod
remain theirs and are used here under **MIT**. The Rust in this repository is a clean-room
rewrite for Horizon, not a translation of the original source.

- `LICENSE` — the upstream licence text.
- `LICENSE-NOTICE.md` — what this port carries and where it came from.
- The upstream project: https://github.com/vectorwing/FarmersDelight

## Technical details

```
horizon-mod.toml   package manifest (id, version, api, [native] pin)
Cargo.toml         the crate; depends on `horizon-mod-api` from the fetched SDK
src/               lib.rs (the one `impl Hooks`), runtime.rs (pure logic + tests), glue
assets/ data/      the content the package ships
tools/             content generators and the e2e scenario
```

- Build: `cargo horizon-mod build` (packages; it does not compile against the game).
- Test: `cargo test`, `cargo clippy --all-targets -- -D warnings`.
- In-game: `ports/tools/launcher-game.sh run farmersdelight` in the campaign tree, or install the
  `.hmod` and play; the harness scenario is `tools/e2e.sh`.

This repository is generated from the campaign's port at `ports/farmersdelight/`; the port's own
`notes.md` is the full record (decisions, traps, evidence).
