# Farmer's Delight — port notes

> **Current state: the SDK package (`api = "0.2"`, version `0.2.0`, 2026-11-18).** Read
> [`## Migration to the SDK format`](#migration-to-the-sdk-format-2026-11-18) at the end first: the
> file map, the helper→hook table, the measured ladder, the gaps and this port's own traps.
> Everything before it describes the retired `.hnmod` container (Horizon `1d63e0a5`, `mixins/`,
> `content/`, the staging ladder) and is kept as the record, not as instructions.

Ported from **Farmer's Delight 1.21.1-1.3.4** (`vectorwing/FarmersDelight`,
branch `1.21`, commit `dd8b9b0bfec916a2da88a6d64e3a37f4a02c3259`) for
Minecraft Java 26.3 on Horizon **c9d474d** / horizon-mixins **d9c83d7**.

**Licence: MIT** (`LICENSE`, "Copyright (c) 2020 vectorwing"). The port is
**local-only until the maintainer approves publication** (ARCHITECTURE.md §8).
Full source evidence is in
[`../_shared/upstream/farmersdelight/FINDINGS.md`](../_shared/upstream/farmersdelight/FINDINGS.md).

Status: **slice 1 implemented and verified in-game** — the **77 food items**
and their nutrition/saturation, stack size, use remainder, consume time and
vanilla status effects, plus 45 carried recipes, the `c:`/mod item tags, the
English names and one creative tab. The cutting board, cooking pot, skillet,
crop blocks, feasts blocks and the mod's own `nourishment` effect are deferred.

## Scope: slice 1

The slice is every item `ModItems` registers with a `FoodProperties`:
`FoodValues.java` and `ModItems.java` give 77 registrations (raw crops, the
foodstuffs, sweets, handheld meals, soups/stews, plated meals, feast portions
and dog food). The table below is the contract pinned in
[`mixins/farmersdelight/runtime.rs`](mixins/farmersdelight/runtime.rs); the
content generator writes the same values into the `.hmod`.

| group | examples | nutrition / saturation |
|---|---|---|
| raw crops | cabbage, tomato, onion | 2/1.6, 1/0.6, 2/1.6 |
| foodstuffs | fried egg, beef patty, cooked bacon, smoked ham | 4/3.2 … 10/16.0 |
| sweets | cake slice, pie slices, cookies, popsicle, custard | 2/0.4 … 7/8.4 |
| handheld meals | egg sandwich, hamburger, dumplings, kelp roll | 5/5.0 … 12/12.0 |
| soups/stews | cooked rice, beef stew, pumpkin soup, bone broth | 6/4.8 … 14/21.0 |
| plated meals | bacon and eggs, ratatouille, grilled salmon | 10/12.0 … 14/21.0 |
| feast portions | roast chicken, stuffed pumpkin, shepherd's pie | 14/21.0 |
| pet food | dog food | 4/1.6 |
| drink | apple cider | 0/0.0, always edible, absorption |

**Unit conversion:** 1.21.1 `FoodProperties.Builder.saturationModifier(m)`
stores `FoodConstants.saturationByModifier(nutrition, m)` = `nutrition * m *
2`; 26.3's `minecraft:food.saturation` is that absolute value. `KELP_ROLL`
uses the direct `new FoodProperties(12, 12, false, 2.4f, …)` constructor, so
its saturation stays **12** (not `12 * 0.5 * 2`) and its use is **2.4 s**.
`fast()` is **0.8 s**, otherwise **1.6 s**.

## Content adaptations (26.3)

The generator is [`tools/gen_content.py`](tools/gen_content.py). It writes:

1. **Item declarations** — `data/farmersdelight/horizon/item/<name>.json`,
   all copying `minecraft:apple` (which supplies `consumable`, `food`,
   `use_effects`, the 64 stack and the item model/name rename the content
   loader performs). Each overrides:
   - `minecraft:food` with `nutrition` and the absolute `saturation`, plus
     `can_always_eat` for apple cider, the melon popsicle and glow berry
     custard;
   - `minecraft:consumable` with `consume_seconds: 0.8` (fast foods),
     `consume_seconds: 2.4` (kelp roll), the drink animation/sound (apple
     cider and bone broth) and `on_consume_effects` (vanilla effects);
   - `max_stack_size: 16` for bowl foods, drinks and the custard;
   - `minecraft:use_remainder` (`minecraft:bowl` or
     `minecraft:glass_bottle`);
   - `!minecraft:compostable`, because the slice carries no compost values
     and apple's 30 % would be wrong.
2. **Assets** — `assets/farmersdelight/items/<name>.json` model definitions,
   the upstream item models (`generated`, `handheld` for barbecue stick/ham/
   smoked ham, `mug` for apple cider), the upstream MIT textures and the
   English item names.
3. **Creative tab** — `data/farmersdelight/horizon/creative_tab/farmersdelight.json`
   with `itemGroup.farmersdelight`, the hamburger icon and all 77 ids in
   upstream registration order.
4. **Recipes** — 45 vanilla crafting/smelting/smoking/campfire recipes whose
   result is a slice food and whose ingredients exist in the slice or vanilla.
   The 23 `farmersdelight:cooking` and all `farmersdelight:cutting` recipes are
   deferred with their stations; the three crate-return recipes are skipped
   because the crates are not in the slice.
5. **Tags** — 26.3 ships no `c:` tags, so the port supplies the 21 the carried
   recipes name (`c:foods/*`, `c:crops/*`, `c:eggs`, `c:drinks/milk`), plus the
   mod's `meals`, `snacks`, `sweets` and `drinks` item tags filtered to the
   slice.

## Effects

The vanilla effects on the foods are carried through
`minecraft:consumable.on_consume_effects`:

| effect | foods |
|---|---|
| `minecraft:hunger` 600 t, 30 % | wheat dough, raw pasta, chicken cuts |
| `minecraft:speed` 400 t | cake slice |
| `minecraft:speed` 600 t | the four pie slices |
| `minecraft:regeneration` 100 t | fruit salad, mixed salad |
| `minecraft:nausea` 240 t, 30 % | nether salad |
| `minecraft:glowing` 100 t | glow berry custard |
| `minecraft:absorption` 1200 t | apple cider |

The mod's own `nourishment` effect (on cooked rice, stews, soups and plated
meals) needs a registered mob effect and a hunger-tick hook, so it is
**deferred**: those meals carry their nutrition only. This is recorded, not
silently dropped.

## Runtime

`mixins/farmersdelight/runtime.rs` is a **test-only** model: the whole module
is `#[cfg(test)] mod tests`, so the copy injected into the game is empty in a
normal build and the crate lib carries no dead code. It pins the 77 ids, the
nutrition/saturation table, stack sizes, remainders, consume times, always-
edible flags, effects and creative-tab order. There are **13 tests**; no fake
hook or content-shaped test was added. The bundle has **no `[[injection]]`**:
slice 1 is content-only, so the deliverable is the `.hmod`.

## Gaps and deferred work

- **Crop blocks and seeds**: cabbage/tomato/onion crops, wild crops, rice and
  the rich soil/farmland are deferred. The three raw crop items exist as food
  but cannot be farmed yet.
- **Cutting board, cooking pot, skillet and their recipe types**: all
  `farmersdelight:cutting` and `farmersdelight:cooking` recipes are deferred,
  so foods whose only source is a knife or pot (bacon, cod/salmon slices,
  mutton chops, most stews) are not survival-obtainable in this slice.
- **Feast blocks** (roast chicken block, stuffed pumpkin block, …) and the
  placeable pies/cheesecakes are deferred; only their portion items ship.
- **`nourishment`**: the custom effect is not registered (see above).
- **Non-food drinks**: milk bottle, hot cocoa and melon juice have no
  `FoodProperties` upstream (they clear effects/heal), so they are outside
  "food items and their nutrition" and are deferred.
- **Interactions**: dog food feeding wolves and horse feed are deferred (they
  are `interactLivingEntity` code, not food data). The dog food item still
  carries its own 4/1.6 food values. The melon popsicle's `clearFire` has no
  26.3 consume-effect type and is deferred; its 3/1.2 always-edible food is
  carried.
- **Compost values, recipe advancements, tooltips and client effect tooltips**
  are not carried; they are presentation/data-map extras.
- **Not exercised in-game**: the vanilla consume effects, the bowl/bottle
  remainder returns, the drink animation and the always-edible foods are
  pinned by the runtime tests and the content check, but the e2e eats the
  no-effect fried egg and checks nutrition/saturation only.
- **Config**: `Configuration.ENABLE_FOOD_EFFECT_TOOLTIP` and the rest of the
  upstream config are client/tooltip behaviour and are deferred.

## Files

```
Cargo.toml, Cargo.lock, compatibility.json, rust-toolchain.toml, LICENSE
mixins/farmersdelight/horizon-mixin.toml
mixins/farmersdelight/runtime.rs
tools/gen_content.py, tools/horizon.py, tools/tab_check.py,
tools/craft_check.py, tools/e2e.sh
content/horizon-mod.toml
content/assets/farmersdelight/**      (items, models, textures, lang)
content/data/farmersdelight/**        (horizon/item, horizon/creative_tab,
                                      recipe, tags)
content/data/c/tags/item/**           (the 21 c: tags)
content/farmersdelight.hmod           (delivered)
```

Skills: superpowers/using-superpowers, test-driven-development, systematic-debugging, verification-before-completion; BeautifulRust/rust-best-practices

## Verification

Runs on 2026-11-05, Linux, Rust 1.95.0, Horizon `c9d474d` / horizon-mixins
`d9c83d7`, 26.3 prepared resources at
`/home/adlf/Desktop/horizon/.local/horizon-client/26.3`. Every cargo/stage/e2e
command ran with `CARGO_INCREMENTAL=0` and `CARGO_BUILD_JOBS=1` or `2`.

### Standalone crate

| Check | Command | Result |
|---|---|---|
| fmt | `cargo fmt --all -- --check` | exit 0 |
| clippy | `cargo clippy --all-targets --locked --offline -- -D warnings` | exit 0, **0 warnings** |
| tests | `cargo test --locked --offline` | exit 0, **13 passed**, 0 failed |

### Staged pin (`stage → check → test → build → ctl`)

| Step | Result |
|---|---|
| stage | exit 0, revision `c9d474dd…`, 1 module, **0 injections** (`target/mixins/plan.json`); re-staged in 235 s after a campaign-wide `clean-targets.sh` removed `target/mixins` |
| check (direct cargo) | `cargo check --locked --offline -p horizon-client --bin horizon-client --no-default-features --target-dir …/target/horizon-cargo` in `target/mixins/modded`: **exit 0**, **0 warnings**, 1 m 07 s (`target/direct-check.log`) |
| test (direct cargo) | the same staged tree with `game::farmersdelight::tests -- --nocapture`: **exit 0**, **13 passed; 0 failed; 531 filtered out**, 24 m 17 s (`target/direct-test.log`); the resulting test binary re-ran in 0.14 s with the same 13/13 after the re-stage |
| `horizon.py check` / `test` wrappers | the wrapper runs were started, but the machine was saturated by 16–18 concurrent cargo builds from other ports (load average 34–47, 14 GiB nearly full) and the wrapper's cargo command was still compiling when the 240–900 s timeouts expired; no `cargo_exit` was recorded. The direct commands above are the same cargo invocations (the `tools/horizon.py` test filter was changed from `game::example_mod::tests` to `game::farmersdelight::tests`) |
| build / ctl | **not run** — the port's `target/horizon-cargo/fast` was never built; a fast build under that load risked the OOM/reboot the campaign notes warn about. The in-game e2e below used the same-pin `measurements` fast client/ctl (`c9d474d`); this bundle has **no injections**, so the game binary is equivalent for content verification. The own build remains the one open ladder step |

**Environment finding:** another job ran the campaign-wide
`ports/tools/clean-targets.sh` while this port's stage was live; it removed
`target/mixins` (and SIGTERMed a live build). The stage was recreated and the
direct test re-run. The wrapper `horizon.py test` also needs the staged
`game::farmersdelight::tests` filter (fixed here); the template ships
`game::example_mod::tests`.

### Content

| Check | Command | Result |
|---|---|---|
| build | `cargo-horizon-mod build` (pinned tool, from `content/`) | exit 0, `target/horizon-mod/farmersdelight.hmod` |
| check | `cargo-horizon-mod check content/farmersdelight.hmod --json` | `{"ok":true,…}`: id `farmersdelight`, version `1.21.1-1.3.4`, API 0.2, both, registries, unsigned, **382 files** |
| delivered | `content/farmersdelight.hmod` | **134,336 bytes**, SHA-256 `7f1fa2bbacd05dbc6eee1c0cc03304910b2885b88f4b9e7574f0803d41d78a47` |

### In-game e2e (`tools/e2e.sh`)

Run with `CTL`/`GAME` pointing at the `measurements` fast binaries (same pin
`c9d474d`) because this port's fast build was blocked by the load; the content
mod itself is this port's `content/`.

| Run | Result |
|---|---|
| default | **exit 0, `PASS`**: 9/9 representative foods give, the creative tab lists **77/77**, both recipes craft (`bread + two fried eggs → egg_sandwich`; `three wheat + one egg → 3 wheat_dough`) and `bread + stick` is empty, hunger drains to 0, the fried egg takes food **0 → 4** and saturation **0 → 3.2**, a stick leaves both unchanged, and `farmersdelight:not_a_food` is refused (`/tmp/opencode/fd-e2e-pre2.log`) |
| negative (`NEGATIVE=1`) | **exit 1, as required**: `FAIL: NEGATIVE run did not observe the false assertion (fried egg gave 4 food, expected the wrong 5)` (`/tmp/opencode/fd-e2e-negative.log`) |

**Harness finding:** `--creative` opens the creative inventory, which has no
2x2 grid; the e2e switches to survival (`gamemode survival @s`) before
`tools/craft_check.py` and stays there for the eat phase. The hunger drain uses
`effect give @s minecraft:hunger 120 255`, which reaches food 0 in ~60 ticks.

## Re-pin to `41f822b` + the crops/seeds slice (2026-11-06)

**Class/tier:** content / **machines** (`ports/ledger.csv`; the slice itself is
a `data`+`logic` shape per `horizon/docs/modpacks.md`: two crop blocks on
vanilla crop templates carry growth/light/support/bone meal, and one native
`use_item_on` hook plants a crop from its seed because the crop blocks have no
item of their own).

**Migrated:** Horizon `41f822b659789780f58d32c63ddfb6887ea1b8bf`, horizon-mixins
`5a64af938ec86b00d53d16a408aabc0038fb3e57`; `tools/horizon.py` uses `--lib`.

**Feature:** `farmersdelight:cabbages` (`minecraft:wheat` template, 8 ages) and
`farmersdelight:budding_tomatoes` (`minecraft:beetroots`, 4 ages), with the
upstream blockstate/model/texture stages, `cabbage_seeds`/`tomato_seeds` items
and seed-shaped item definitions, age-keyed 26.3 loot tables (ripe = produce +
seeds, young = seeds only), the pickaxe tag where upstream has it, names and the
creative-tab entries. `use_item_on` plants `cabbages`/`budding_tomatoes` when
the matching seed is used on the top face of farmland with air above, spends one
seed and claims the click; `server_tick` re-sends a guest's hand.

**Deviations:** the onion crop is not shipped (upstream plants it with the
`onion` food item, which has no content shape for a second block-item role);
rice and the wild crops, the rich soil/farmland and the crop-related
advancements are deferred. No cooking pot/cutting board/skillet/juices.

### Verification state (2026-11-06, pin `41f822b`)

| Step | Result |
|---|---|
| runtime tests | `cargo test --locked --offline`: **15 passed**, 0 failed (13 food + 2 crop tests) |
| staged check | exit 0, but **5 warnings** recorded before the final `private_interfaces`/dead-code attributes; needs one re-run for 0 |
| staged embedded test | exit 0, **15 passed**, same pre-fix warning caveat |
| content build | exit 0, `cabbage_seeds`+`tomato_seeds`+the two crops, **147,137 B** `.hmod` |
| content check | exit 0, SHA-256 `081378989c1b8f5603b561f01f3af4d55e0c5255bce33d36b626528223e18f53` |
| fast build | the binary was produced (103,100,432 B, 18:29) but the wrapper reported exit 1 when the session was interrupted |
| e2e | the crop phases ran to the creative-tab step; the tab check used the old cache path (`<home>/.horizon-cache`), while the pin writes `<scratch>/.horizon-cache`; `tools/e2e.sh` is fixed to find either. The re-run was killed by the OOM-thrashing machine before the crop phase |

Next: re-run `check → test → build` in one slot (expect 0 warnings), then
`tools/e2e.sh`; the crop phases assert planting, `randomTickSpeed 0` vs `300`
growth, the stone/vanilla-wheat controls and the ripe-vs-young drops.

## Content embedded in the bundle; re-verified in game (2026-11-10)

**Beat: one file per project.** The registry takes one `file` per version, so a
port that ships a `.hmod` *and* a `.hnmod` cannot be listed faithfully. This port
now follows the AE2 pattern proved on `charginggadgets` (PR #526): the built
content mod travels **inside** the bundle at
`mixins/farmersdelight/data/farmersdelight-content.hmod`, and `mixins/farmersdelight/embedded_content.rs`
compiles it in with `horizon_client::native_data!` and injects it at
`content_root` (`target = "horizon_client::game::mods::content_root"`,
`kind = "HEAD"`, `cancellable = true`). The helper writes the file to
`.horizon-cache/farmersdelight-embedded-content.hmod` once, appends it to the dev-mod list
and returns the merged root; the recursive call stops at its own
`dev.contains(&file)` guard, so the vanilla body runs once. The `.hmod` stays in
`content/` as the build input; `tools/package.py` refuses a stale copy in `data/`.

**Proved with no `--dev-mod`:** the e2e launches the game *without* passing the
content folder, so every content id and every behaviour it asserts comes from the
bundle. `EMBEDDED=1` is the flag; the launch line passes an empty dev-mod list.

| Artifact | Size | SHA-256 | Entries |
|---|---:|---|---:|
| `content/farmersdelight.hmod` (input) | 147,137 B | `081378989c1b8f5603b561f01f3af4d55e0c5255bce33d36b626528223e18f53` | 418 |
| `farmersdelight.hnmod` (deliverable, embeds it) | 81,502 B | `1f526bc750485a9c` | 8 |

E2E (one lease each, `CARGO_INCREMENTAL=0`, shared `1d63e0a5` `horizon-ctl`, the
port's own fast game): default `FAIL` + `EXIT:1` at “the cabbage never grew”; `NEGATIVE=1` `FAIL` + `EXIT:1`.

`python3 ports/tools/check-e2e-logs.py farmersdelight --strict` → every log carries its
own `PASS`/`FAIL` + `EXIT:` pair. `python3 ports/tools/upstream_ready.py farmersdelight` →
**BLOCKED — e2e has no PASS log**.

### The crop growth gap that blocks the e2e (measured 2026-11-10) — **superseded 2026-11-11, see the next section**

The content embeds and loads from the bundle (the e2e reads the 79-item creative
tab, crafts, eats, plants both crops and, in survival, spends exactly one seed —
all with no `--dev-mod`). The remaining failure is the **growth phase**:

```
(cabbage planted, age=0)  … gamerule randomTickSpeed 1000 … 1000 ticks …
FAIL: the cabbage never grew: farmersdelight:cabbages[age=0]
```

What was measured, in order:

1. A mod crop on the `minecraft:wheat` / `minecraft:beetroots` template does
   **not** grow from the template's own random tick at this pin (200 and 1000
   ticks at speed 300/1000, light 15, chunk loaded, player adjacent).
2. A `random_tick` (API 33) injection for the crop blocks never fired either: the
   port-side growth slice (an `age` roll in `runtime.rs` plus a
   `level_tick_post` write) left the crop at `age=0`, so the hook is not
   dispatched for a content block that the behaviour table does not mark as
   randomly ticking. That slice was removed again rather than shipped dead.
3. The earlier `minecraft:air` readings were **mob trampling** (a mob walking on
   the farmland turns it to dirt and breaks the crop during a 1000-tick wait);
   `gamerule mobGriefing false` keeps the crop alive but does not make it grow.

Needed: a stamped way for a content crop to be randomly ticked (`Class::Crop`
carrying the random-tick flag into the mod block's behaviour row, or a hook that
dispatches `random_tick` for every mod block), or a documented engine feature the
port can name. Until then this port stays `blocked(crop-growth)`; everything else
in the slice is verified.

## Crop growth unblocked (2026-11-11): the blocker was the harness, not the engine

**Result: the crops grow.** All three e2e legs are green (default, `NEGATIVE=1`,
`EMBEDDED=1` with its unusable-cache control). `random_tick_speed 1000` grows a
planted cabbage from age 0 while `random_tick_speed 0` does not, and a ripe
cabbage drops produce + seeds where an age-0 one drops seeds only.

### Root cause, measured in this order

1. **The content crop already random-ticks.** A cabbage on the `minecraft:wheat`
   template is grown by the engine's `Class::Crop` random tick
   (`crop_random_tick`, `horizon-world/src/level/tick.rs:853`: light ≥ 9,
   farmland growth speed, `1/(25/speed + 1)`, age + 1) — the vanilla rule
   upstream's `CabbageBlock extends CropBlock` uses. `mods.md`'s `random_ticks`
   content flag (`horizon-mods/src/content.rs:120`) makes that tick explicit; a
   probe run with the flag removed from both block files grew the same crop to
   age 2, so the flag is declarative here, not load-bearing.
2. **The e2e set the gamerule with a name the pin refuses.**
   `gamerule randomTickSpeed 1000` answers
   `["Incorrect argument for command","gamerule randomTickSpeed<--[HERE]"]` — the
   pin's rule ids are snake_case (`horizon-client-core/src/gamerules.rs:108`,
   `random_tick_speed`; `mob_griefing` likewise). `ctl command` replies
   `ok:true`, so the e2e's `>/dev/null` swallowed the refusal: random ticks
   stayed at 3 (~0.07 draws per cell per 1000 ticks) and the crop never grew.
   The same defect left mob griefing on, which is what turned a planted crop
   into `minecraft:air` in the 2026-11-10 runs.
3. **The farmland was not hydrated.** `Class::Farmland` (`tick.rs:994-1026`)
   decays moisture and, at 0, turns the farmland to dirt when the block above is
   not in `minecraft:maintains_farmland`; a *mod* crop is not in that vanilla
   tag. The growth cell now has a water source at its own y (inside the stone
   floor, so it cannot flow), which keeps the farmland moist and gives the 3.0
   soil growth bonus.
4. **The harvest phase read the world instead of the break.** A break's reply
   names the block and the rolled drops (`block_before`, `drops`); the item
   entities are transient — an adjacent player picks a single seed up inside
   ten ticks. The phase now asserts the drop map, then requires the seed to be
   in the inventory or on the ground, and the young-crop break stands next to
   the block it breaks (a `break` out of reach drops nothing).

### Upstream source read for this slice

`vectorwing/FarmersDelight`, branch `1.21`, commit
`dd8b9b0bfec916a2da88a6d64e3a37f4a02c3259` (the port's own version
1.21.1-1.3.4; the clone was `--depth 1` in `/tmp` and is deleted):
`common/block/CabbageBlock.java` (`extends CropBlock`, only a shape and the
seed id), `BuddingBushBlock.java` (`AGE` 0..4, `MAX_AGE` 3, `isRandomlyTicking`
always true, vanilla growth-speed roll, `growPastMaxAge`),
`BuddingTomatoBlock.java` (converts to `tomatoes` at max age, bonemeal 1..4)
and `TomatoBlock.java` (`VINE_AGE` 0..3, fixed growth speed 5, rope climbing).
The rewrite keeps the engine's `Class::Crop` growth for the cabbage (identical
to upstream) and does not re-implement vanilla's growth maths.

### What the port ships for it

- **Content:** `"random_ticks": true` on `cabbages.json` and
  `budding_tomatoes.json` — the pin's documented way to say a content block
  ticks (`mods.md` §"Content mods"). Measured as not required (the template tick
  already applies); declared so the port does not depend on template
  inheritance and so the budding bush ticks at every age as upstream's does.
- **Code:** `/fdcrop setage <x> <y> <z> <age>` — API 8
  `register_commands`/`perform_command` plus `Game::native_set_block` (API 24).
  It is the brief's experiment (a) and a **harness diagnostic, not an upstream
  command**: the e2e writes `age=1` into a planted `age=0` cabbage, reads it
  back with `ctl block`, resets it to 0, and shows `age=9` refused with
  `has ages 0..=7`. The pure parser (`parse_command`, `AgeWrite`,
  `CropCommand`) is TDD'd in `runtime.rs` (5 tests: valid forms, negative
  coordinates, another mod's command, malformed lines, an age outside a byte).
- **No growth maths in Rust.** Growth, light, support and bone meal stay the
  engine's `Class::Crop` path (the re-pin audit's F1 route). The earlier
  port-side `random_tick`/`level_tick_post` growth slice stays removed.

### Deviations

- `/fdcrop setage` is a port-only diagnostic command: it writes only this mod's
  crops, refuses an age past the crop's top, and changes nothing else.
- `random_ticks: true` is declared although the wheat/beetroots template already
  ticks (same behaviour either way, measured).
- The e2e harness corrections: snake_case gamerules read back after every set
  (`set_rule`), a hydrated growth cell, a reach-correct position for the
  young-crop break, retried `ctl give` reads, the break-reply drop map, and the
  `EMBEDDED` unusable-cache control leg.

### Gaps (this slice changed none of the earlier ones)

- The tomato is a two-block plant upstream: `budding_tomatoes` (ages 0..4, with
  `growPastMaxAge` converting into `tomatoes`) and then `TomatoBlock`
  (`VINE_AGE` 0..3, rope climbing, hanging tomatoes). This slice still ships
  `budding_tomatoes` only (ages 0..3 on the beetroots template), so a mature
  budding bush stays at age 3 instead of becoming a tomato vine; the rope and
  hanging-tomato forms are deferred with it. Recorded, not faked.
- Onion, rice, wild crops, rich soil/farmland and the crop advancements remain
  deferred (see the gaps list above).
- Bone meal on a mod crop uses the template's `Class::Crop` bonemeal path; the
  e2e does not exercise it (upstream's cabbage bonemeal is vanilla's).
- The budget of a random tick on a content crop is the engine's, not a hook:
  a future pin that stops inheritance would need `random_ticks` (already
  declared here).

Skills: superpowers/using-superpowers, brainstorming, writing-plans, test-driven-development, systematic-debugging, verification-before-completion; BeautifulRust/rust-best-practices, rust-port

### Verification (2026-11-11/12, pin `1d63e0a`, mixins `5a64af9`)

| Step | Command | Result |
|---|---|---|
| fmt | `cargo fmt --all -- --check` | exit 0 |
| clippy | `cargo clippy --all-targets --locked --offline -- -D warnings` | exit 0, **0 warnings** |
| crate tests | `cargo test --locked --offline` | **20 passed**, 0 failed |
| staged check | `python3 tools/horizon.py check` (slot, forwarded) | `cargo_exit 0`, **0 warnings**, 2 m 08 s (`target/ladder/check.log`) |
| staged test | `python3 tools/horizon.py test` | `cargo_exit 0`, **20 passed; 0 failed; 1240 filtered out** (`target/ladder/test.log`) |
| fast build | `python3 tools/horizon.py build` | `cargo_exit 0`, **0 warnings**, `fast` profile 12 m 44 s (`target/ladder/build.log`) |
| content + bundle | `tools/package.sh` (twice) | `.hmod` **147,172 B** `6222ca9d…`, 418 entries; `.hnmod` **84,439 B** `cd453acd…`, 10 entries; identical bytes on the second run |

In-game legs, one lease each through `ports/tools/build-slot.sh e2e` (shared
`1d63e0a5` `horizon-ctl`, this port's own fast game, `CARGO_INCREMENTAL=0`):

| Leg | Log | Result |
|---|---|---|
| default (`--dev-mod content/`) | `ports/_shared/evidence/farmersdelight/e2e.log` | **PASS + EXIT:0** |
| `NEGATIVE=1` | `e2e-selftest.log` | **FAIL + EXIT:1** (the deliberately wrong food assertion) |
| `EMBEDDED=1` (no `--dev-mod`), `CONTROL=1` | `e2e-embedded.log` | **PASS + EXIT:0**; the control launch with the embedded file unusable reports `Unknown item 'farmersdelight:cabbage'` and `Unknown block type 'farmersdelight:cabbages'` |

Transcripts from the growth and harvest phases (`e2e.log`): `random_tick_speed 0`
→ `the crop stayed age 0`; `random_tick_speed 1000` → `the cabbage grew:
farmersdelight:cabbages[age=2]` (the loop samples every five ticks, so it can
catch a later age than the first); ripe break
`{"before":"farmersdelight:cabbages[age=7]","drops":{"farmersdelight:cabbage":1,"farmersdelight:cabbage_seeds":1}}`;
young break `{"before":"farmersdelight:cabbages[age=0]","drops":{"farmersdelight:cabbage_seeds":1}}`.

Gates: `python3 ports/tools/check-e2e-logs.py farmersdelight --strict` →
`logs=5 ok=5 missing-exit=0 bad-exit=0`, exit 0;
`python3 ports/tools/check-skills.py --notes ports/farmersdelight` → `SKILLS GATE:
PASS`; `python3 ports/tools/upstream_ready.py farmersdelight` → exit 0 (it still
prints `blocked(crop-growth)` from the ledger row, which the orchestrator
updates; this slice does not edit the ledger or `ARCHITECTURE.md`).

The raw probe transcripts (growth with and without the content flag, the refused
camelCase gamerule in The End and the Overworld, the dry-farmland control) were
written under `/tmp/opencode/fd-probe/` and lost when the machine restarted; the
facts they measured are quoted above and the same behaviour is asserted durably
by the e2e.

## Registry packaging: version `0.1.0` (2026-11-12)

The registry takes one file per project, so this port ships the **native bundle**
with its content embedded (the AE2 pattern, previous section). The entry, the
file and the release must name the **port's** version, not the original mod's
(reviewer feedback on an earlier submission, `AGENTS.md` §1.7), so the version
moved from `1.21.1-1.3.4` to **`0.1.0`** everywhere the packaging reads it:

| File | Change |
|---|---|
| `content/horizon-mod.toml` | `version = "0.1.0"` (the `.hmod` manifest) |
| `mixins/farmersdelight/horizon-mixin.toml` | `[mod] version = "0.1.0"` (copied into the bundle's `horizon-native.json`) |
| `tools/gen_content.py` | writes `version = "0.1.0"`, so a regeneration agrees |
| `Cargo.toml` / `Cargo.lock` | crate `horizon-farmersdelight` `0.1.0` (the source tree no longer claims the upstream version) |

`unzip -p farmersdelight.hnmod horizon-native.json` → `"version": "0.1.0"`.

The bump changes both artifact bytes, so everything below was re-run from the new
sources; the scenario is unchanged.

| Check | Result |
|---|---|
| `tools/package.sh` (twice, one slot each) | `.hmod` **147,172 B** `776569326f3e5be24ac5c60d69f964e1c6a8c1332b0bdaf20567312fd8bf0eb4`, 418 entries; `.hnmod` **84,428 B** `84dbf6c52cba00240f094efdff3aa90df7d86a6d94ce55845f29959fd4abc4a1`, 10 entries; `cmp` on both files: **byte-identical** between the two runs; `mixins/farmersdelight/data/farmersdelight-content.hmod` identical to `content/farmersdelight.hmod` |
| `cargo horizon-mod check farmersdelight.hnmod --kind native-mod` | exit 0 — `farmersdelight 0.1.0 (native-mod, format native-1/1d63e0a5…/5a64af93…, for 26.3)`, 84,428 bytes, SHA-256 `84dbf6c5…`, unsigned |
| `cargo-horizon-mod check farmersdelight.hmod` (inside `package.sh`) | exit 0 — `farmersdelight 0.1.0 (mod, API 0.2, both)`, 147,172 bytes, `7765693…`, `no code: content only` |
| e2e default (`--dev-mod content/`) | **PASS + EXIT:0** (`ports/_shared/evidence/farmersdelight/e2e.log`) |
| e2e `NEGATIVE=1` | **FAIL + EXIT:1** — `NEGATIVE run did not observe the false assertion (fried egg gave 4 food, expected the wrong 5)` (`e2e-selftest.log`) |
| e2e `EMBEDDED=1` (no `--dev-mod`), `CONTROL=1` | **PASS + EXIT:0**; the unusable-`.horizon-cache` control still reports `Unknown item 'farmersdelight:cabbage'` and `Unknown block type 'farmersdelight:cabbages'` (`e2e-embedded.log`) |

The scenario is unchanged and still fail-capable in both directions, from the new
content bytes: `random_tick_speed 0` → `the crop stayed age 0`; `random_tick_speed
1000` → `the cabbage grew: farmersdelight:cabbages[age=1]` (the loop samples every
five ticks); ripe break `{"before":"farmersdelight:cabbages[age=7]","drops":
{"farmersdelight:cabbage":1,"farmersdelight:cabbage_seeds":1}}`; young break
`{"before":"farmersdelight:cabbages[age=0]","drops":{"farmersdelight:cabbage_seeds":1}}`;
the creative tab lists 77/77 foods.

Gates: `check-e2e-logs.py farmersdelight --strict` → `logs=5 ok=5 missing-exit=0
bad-exit=0`, exit 0; `check-skills.py --notes ports/farmersdelight` → `SKILLS
GATE: PASS`; `upstream_ready.py farmersdelight` → **READY** (exit 0);
`fix_icons.py` → `every publish tree has a compliant icon.png` (exit 0).

The publish tree (`ports/_shared/horizon-mods/publish/farmersdelight/`) was
re-synced from this port (content, mixins, tools, `Cargo.toml`/`Cargo.lock`, the
`.hnmod`), and its `registry.toml` rewritten: `authors = ["vectorwing",
"AdLf0208"]`, `categories = ["food", "game-mechanics"]`, version `0.1.0`, the
rebuilt `sha256`, an honest summary and a description with the implemented and
not-yet-ported lists. Nothing was submitted and no PR was opened.

Skills: superpowers/using-superpowers, verification-before-completion; BeautifulRust/rust-best-practices (loaded; no Rust source changed — packaging and metadata only)

## Migration to the SDK format (2026-11-18)

> **This is the current state.** Everything above is the history of the retired `.hnmod`
> container (pin `1d63e0a5`, `mixins/`, `content/`, the staging ladder); it is kept as the
> record, not as instructions. None of `mixins/`, `content/`, `compatibility.json`,
> `rust-toolchain.toml`, `tools/horizon.py`, `tools/package.py`, `tools/package.sh` or
> `farmersdelight.hnmod` exists in the port any more — they are under
> [`../_shared/evidence/farmersdelight/retired-hnmod-0.1.0/`](../_shared/evidence/farmersdelight/retired-hnmod-0.1.0/).
> `src/runtime.rs` is **byte-identical** to the retired `mixins/farmersdelight/runtime.rs`
> (sha256 `3ab07b4216e4fdc8072ae93389608e10191aeb87735d2be8618e9ad429a0bee3`), including the
> copy inside the built package.

**Why.** Horizon's mod-API merge (#16941, launcher ≥ 0.2.70; PR #17992 cleans up the old files)
retired the `.hnmod` container, and the registry closed every `.hnmod` submission on 2026-11-18
with one reason: *"republish this as a native `.hmod` with the SDK from the next Nightly"*. The
new unit is **one flat `.hmod`** — content, native code, or both — so this port no longer needs
the AE2 embedded-content trick, its `content_root` injection or its `.horizon-cache` write:
`assets/` and `data/` sit beside `src/` in the same package. Version **0.2.0** supersedes the
closed **PR #817**; nothing else about the slice changed.

**Pins.** SDK nightly **`0b14f8acfd97fa2c750e42795249dcc981178ca4`** (surface + `api-source`
cached at `ports/_shared/sdk/0b14f8a/`, copied into `.horizon/sdk/`), mixins
**`e59da9094f0c2fdda6c627fe9eff22dec6a906f0`** (was `5a64af93…`), `package_format` 6,
`content_api` 0.2, launcher 0.2.78. The old campaign pin `1d63e0a5` does not apply to a package
build.

**Class + tier unchanged:** `content` / `machines` (ledger row); this slice is the `data` half
carried plus the `logic` half (one plant hook and one readout command) that the crop blocks need.
Licence **MIT, rank 0**; local-only until the maintainer approves publishing (ARCHITECTURE §8).

### File map (old → new)

| Old `.hnmod` layout | New SDK package | What happened |
|---|---|---|
| `horizon-mixin.toml` (`[mod]`, `[compatibility]`, 2 `[[module]]`, 5 `[[injection]]`) | `horizon-mod.toml` (id/name/version/api/side/capabilities + `[native]` game/horizon/mixins) | one manifest, the only identity |
| `Cargo.toml` (`[lib] path = "mixins/farmersdelight/runtime.rs"`) | `Cargo.toml` (`[lib] path = "src/lib.rs"`, one dependency: `horizon-mod-api` from `.horizon/sdk/`) | one crate, no workspace |
| `mixins/farmersdelight/runtime.rs` (module + crate lib) | `src/runtime.rs` + `pub mod runtime;` | **byte-identical**, `cp` then `cmp` |
| `mixins/farmersdelight/support.rs` (`farmersdelight_state`, 219 lines) | `src/lib.rs`: the seed `ItemPart` and the `/fdcrop` body | the state was only those two behaviours |
| `use_on.rs`, `tick.rs`, `register_commands.rs`, `perform_command.rs` (one helper per hook) | methods of `impl Hooks` / the Part | no per-hook files |
| `embedded_content.rs` (`content_root` HEAD) | **gone** | code and content are one package |
| `content/` (`assets/`, `data/`, its own `horizon-mod.toml`, `farmersdelight.hmod`) | package root: `assets/`, `data/` | the carried trees moved up **unchanged** (`diff -r` against the retired copy: empty; 263 assets + 154 data files) |
| `mixins/farmersdelight/data/farmersdelight-content.hmod` | **gone** | nothing to embed |
| `compatibility.json`, `rust-toolchain.toml`, `horizon-native.json`, `Cargo.lock` | **gone** | `[native]` is the pin; the launcher's toolchain builds |
| `tools/horizon.py`, `tools/package.py`, `tools/package.sh` | **gone** (evidence folder) | the SDK ladder is `cargo test`/`clippy`/`fmt` + `cargo horizon-mod build`/`check`/`fits` |
| `tools/gen_content.py` | kept, now writes `assets/` + `data/` at the package root and no manifest | regeneration cannot clobber `src/` or the manifest |
| `tools/e2e.sh`, `tools/tab_check.py`, `tools/craft_check.py` | kept; e2e rewritten for a launcher-built game (`GAME=`/`CTL=`/`CONTROL_GAME=`) | the scenario and assertions are format-independent |
| `farmersdelight.hnmod` (84,428 B, 10 entries) | `farmersdelight.hmod` (**166,376 B**, 425 entries) | one file installs both halves |

### Every `[[module]]` and `[[injection]]` → its new home

| Old piece | New place | Mapping |
|---|---|---|
| `[[module]] id = "runtime"` (food/crop contract, 20 tests) | `src/runtime.rs`, `pub mod runtime;` | a plain module now, so the tests run with `cargo test` alone |
| `[[module]] id = "state"` (`support.rs`) | the `impl Hooks` + `SeedPart` in `src/lib.rs` | the `SYNC` static and the tick queue are gone |
| `[[injection]] embedded-content` (`content_root`, HEAD, cancellable) | **none — gap closed by the format** | `assets/`+`data/` load because they are in the package |
| `[[injection]] use-item-on` (API 74, HEAD, `game`/`stack`/`pos`/`face`/`claimed`) | `ItemPart::use_on` on `farmersdelight:cabbage_seeds` and `:tomato_seeds`, attached with `Registrar::item_part` | the moment of *those items* is a Part; the global hook ran for every item and filtered by id, which is the same behaviour with more work. `face` is `[i32;3]` (`[0,1,0]` = up), the write is `LevelRef::set_block(.., NEIGHBORS\|CLIENTS)` (`Ok(true)` only), and the click is answered `InteractionResult::SuccessServer` |
| `[[injection]] server-tick` (API 4, HEAD; re-sent a guest's changed hand) | **none — the SDK does it** | the seed is spent with `ItemPlace::PlayerHand { player, hand }.extract(.., 1)` inside `host.transaction`, which writes the hand and sends the slot; a creative hand gives the seed without losing it, so the old `if game.creative` branch disappeared too |
| `[[injection]] register-commands` (API 8) | `Hooks::register_commands` → `Command::new("fdcrop setage <pos:block_pos> <age:integer>")` | one usage; the three coordinates are the dispatcher's `block_pos`, so `~` resolves as in any vanilla command |
| `[[injection]] perform-command` (API 8, raw `&str`) | `Hooks::run_mod_command` (`CommandInvocation`) | `perform_command` is gone (trap 17): typed `CommandArgumentValue::BlockPos`/`Integer`, `Flow::Cancel` when handled, `Flow::Continue` for another mod's command |

Commands and readouts are unchanged for a player: `/fdcrop setage <x> <y> <z> <age>`, and the
four answers `fdcrop: <crop>[age=N] written|unchanged`, `fdcrop: no crop at [x, y, z]`,
`fdcrop: <crop> has ages 0..=N`, `fdcrop: <crop>[age=N] is not a registered state` are byte for
byte the 0.1.0 strings, so the e2e's assertions still match. `/fdcrop` remains a **harness
diagnostic**, not an upstream feature.

### The ladder (measured 2026-11-18, `ports/_shared/evidence/farmersdelight/sdk-*.log`)

`ports/tools/build-slot.sh` still wraps every step (one lease each, `BUILD_LOCAL=1` because the
SDK crate cache the build needs is on this box: the remote runner answers
`no matching package named 'unicode_names2'`). The command inside is the SDK's, not
`horizon.py`:

| Step | Command | Result |
|---|---|---|
| runtime tests, **before any glue** (`lib.rs` was only `pub mod runtime;`) | `CARGO_INCREMENTAL=0 cargo test --offline` | **20 passed**, 0 failed (`sdk-test-pre-glue.log`) |
| tests with the glue | the same | **20 passed**, 0 failed (`sdk-test.log`) |
| lints | `cargo clippy --all-targets --offline -- -D warnings` | exit 0, **0 warnings** (`sdk-clippy.log`) |
| formatting | `cargo fmt --check` (never `--all`: `.horizon/sdk` is a path dependency) | clean, and `cmp src/runtime.rs <retired>` still identical after `cargo fmt` (`sdk-fmt.log`) |
| package | `rm -f Cargo.lock && cargo horizon-mod build` | `target/horizon-mod/farmersdelight.hmod`, exit 0 (`sdk-build.log`) |
| second build | the same again | **byte-identical**: 166,376 B, sha256 `68d56c69…e471`, `cmp` empty (`sdk-build-second.log`) |
| check | `cargo horizon-mod check farmersdelight.hmod` | `farmersdelight 0.2.0 (native-mod, format native-1/0b14f8a…/e59da90…, for 26.3)`, 166,376 B, unsigned, **exit 0** (`sdk-check.log`, `sdk-check-json.log`) |
| fits | `cargo horizon-mod fits farmersdelight.hmod --build nightly` | `0b14f8a made for`, exit 0 (`sdk-fits.log`) |
| package contents | `unzip -l` | **425 entries**: 263 `assets/`, 154 `data/`, `src/lib.rs`, `src/runtime.rs`, `horizon-mod.toml`, `Cargo.toml`, `LICENSE`, `LICENSE-NOTICE.md`, `README.md`, `uses.json`; **no `Cargo.lock`** (`sdk-entries.log`) |
| carried runtime | `unzip -p farmersdelight.hmod src/runtime.rs \| sha256sum` | `3ab07b42…bee3` = `src/runtime.rs` = the retired file |
| what it uses | `check --uses` (extracted to `sdk-uses.json`) | **163 entries**: 12 hook entries (4 hooks: `use_on`, `register_commands`, `run_mod_command`, `command`), 18 methods, 2 parts, 1 capability (`registries`), 18 traits, 77 types, 33 handles, 2 macros (`sdk-uses-summary.txt`) |

The manifest inside the archive still carries `side = "both"` and
`capabilities = ["registries"]` (quoted from `unzip -p`); `check --json` reports
`"capabilities":[]`/`"side":null` because that report describes *native* capabilities, not the
content manifest — `uses.json` carries `capability:registries`.

### Traps new to this port (each cost a run here)

1. **The remote runner cannot see the SDK crate cache.** A forwarded `cargo test` on the far box
   answered `error: no matching package named 'unicode_names2' found (required by horizon-core)`
   in offline mode; the SDK's dependencies are cached on this box only. Use `BUILD_LOCAL=1` for
   every `farmersdelight` SDK step until the far box is warmed.
2. **A wrapped `cd ports/<id>` fails on a forwarded run.** The runner already runs in the port's
   mirrored directory (or at the campaign root locally), so `bash -c 'cd ports/farmersdelight && …'`
   answered `cd: ports/farmersdelight: No such file or directory`. Either run with the port as the
   working directory and no `cd`, or pass `BUILD_LOCAL=1` and keep the brief's form.
3. **`tee` hides the cargo exit status.** `cargo test … | tee log` returns `tee`'s status, which
   is 0 — the timeout'd remote run reported `EXIT:0` over a compile error. Every wrapped command
   here starts with `set -o pipefail` (or `set -euo pipefail`).
4. **`CommandInvocation.dimension` is a `DimensionKey`, not a `LevelDimension`.** There is no
   `.key()` on it and `mod_api::dimension_id` is not exported (`__private`); a `LevelRef` compares
   its own `level.dimension()` with it, so `list_levels().find(|l| l.dimension() == command.dimension)`
   is the supported lookup.
5. **`ItemPlace` is the creative rule.** The old `if game.creative { return; }` branch is not
   needed: `extract` on a creative hand gives the item without losing it, which is exactly
   vanilla's `BlockItem` behaviour and what the e2e asserts (2 seeds stay 2 in creative).
6. **The seed click is a Part, not a global hook.** Attaching one `ItemPart` to two items is the
   SDK's answer to "run this for these items"; the retired injection ran for *every* item and
   filtered by id. Same behaviour, fewer calls — but the Part is only asked once the block let the
   click through, the same order vanilla uses for `BlockItem.useOn`, so a chest beside farmland
   still opens.
7. **`gen_content.py` used to `rmtree` its output and write a manifest.** With `--out` being the
   package root that would delete `src/` and the real `horizon-mod.toml`; it now replaces only
   `assets/` and `data/` and writes no manifest.
8. **`SELFTEST`/`EMBEDDED` are gone from the harness.** One `.hmod` installs both halves, so
   there is no embedded file to break: the "ids unknown" control is a *different* launcher build
   (`CONTROL_GAME`), and `SELFTEST=1` is kept as the old name for the deliberately-failing leg.

### Gaps (this migration changed none of the slice's)

- **No in-game run of 0.2.0.** `horizon-launcher` is not installed in this environment, so no
  `0b14f8a` game carrying the package exists; the brief forbids claiming an e2e. The 0.1.0 logs
  (`ports/_shared/evidence/farmersdelight/e2e*.log`) are **historical** evidence for the same
  behaviour transported through a different glue, and `tools/e2e.sh` is updated and waiting for a
  launcher build (`GAME=`, `CTL=`, optional `CONTROL_GAME=`, `SELFTEST=1`).
- **Part scope vs global hook**: `use_on` is attached to the two seed items, so the port no longer
  sees a seed click on a *non*-farmland block at all — it did not act on one before either, so
  nothing observable changed.
- **`/fdcrop setage` age is a signed integer now.** A line like `… -1` or `… 300` used to answer the
  usage text (the old parser read a `u8`); it now answers `fdcrop: <crop> has ages 0..=N`, which is
  the same refusal a player sees for `9`. Recorded as a deviation, not a defect.
- All the earlier content gaps stand unchanged: cutting board, cooking pot, skillet, feasts, the
  tomato vine, the onion/rice/wild crops, `nourishment`, the non-food drinks, pet feeding, compost
  and tooltips (see "Gaps and deferred work" above).

Skills: superpowers/using-superpowers, brainstorming, writing-plans, test-driven-development, systematic-debugging, verification-before-completion; BeautifulRust/rust-best-practices, rust-port
