# Farmer's Delight (Horizon port)

A Horizon port of [Farmer's Delight](https://github.com/vectorwing/FarmersDelight)
by vectorwing — MIT.

**Status: slice 2.** What is below is what the port actually does today; the rest
of the mod is the gap list at the end. One file, `farmersdelight.hmod`, carries
both the content and the code (Horizon mod API `0.2`).

## What works

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

- The **cutting board, cooking pot and skillet**, and every
  `farmersdelight:cutting` and `farmersdelight:cooking` recipe, so the foods whose
  only source is a knife or a pot are not survival-obtainable yet.
- The **tomato vine**: a mature budding bush stays at its last age instead of
  becoming a `tomatoes` vine with rope and hanging tomatoes.
- The onion and rice crops, the wild crops, rich soil and farmland, the feast
  blocks and placeable pies, the mod's own `nourishment` effect, the non-food
  drinks, dog-food feeding and horse feed, compost values and tooltips.

The package also registers one diagnostic command, `/fdcrop setage <x> <y> <z>
<age>`, which writes an age into this mod's own crop blocks and nothing else; it
exists for the port's in-game test and is not an upstream feature.

## Install

Pick `farmersdelight.hmod` in the launcher's mod browser or drop it into the
instance's mod folder. One file installs the items, the crops and the code; the
launcher compiles `src/` against the pinned game build.

## Build from source

```sh
cargo test                 # the pure contract in src/runtime.rs
cargo clippy --all-targets -- -D warnings
cargo fmt --check          # not --all: .horizon/sdk is a path dependency
cargo horizon-mod build    # -> target/horizon-mod/farmersdelight.hmod
cargo horizon-mod check farmersdelight.hmod
cargo horizon-mod fits farmersdelight.hmod --build nightly
```

`.horizon/sdk/` is fetched by any `cargo horizon-mod` command and is never
packed; `Cargo.lock` is not packed either (remove it before a release build).

## Compatibility

Mod API `0.2`; the native half is pinned in `horizon-mod.toml` to game `26.3`,
Horizon SDK build `0b14f8acfd97fa2c750e42795249dcc981178ca4` and mixins
`e59da9094f0c2fdda6c627fe9eff22dec6a906f0`.

## Credits and licence

Original mod: [vectorwing/FarmersDelight](https://github.com/vectorwing/FarmersDelight)
(MIT, Copyright (c) 2020 vectorwing). Port: AdLf0208. Licence: MIT — see
[LICENSE](LICENSE); the original notices are preserved and traced in
[LICENSE-NOTICE.md](LICENSE-NOTICE.md).

The port writes no gamerule or other world setting. As with any content mod, its
blocks and items stay in an existing save if the mod is removed.

## Contributing

The gap list above is the work left. Issues and pull requests are welcome.
