# Farmer's Delight — re-pin audit (read-only, no code changed, no builds)

Target pin: **Horizon `83ad384f`** (427 hooks / 159 helpers) at
`ports/_shared/pins/horizon-83ad384f`, mixins **`e8cd0aa`** at
`ports/_shared/pins/horizon-mixins-e8cd0aa`. Port is currently pinned at
`c9d474d` / `d9c83d7` (`compatibility.json`).

Method: read `notes.md` §Gaps and ARCHITECTURE item 33 (plus the upstream
`FINDINGS.md` behaviour notes), then searched `native-hooks.json` and
`crates/horizon-client/src`, `crates/horizon-world/src`,
`crates/horizon-items/src` at the new pin. No port code, bundle, content or
compatibility file was modified; nothing was built or run in-game.

**Evidence baseline.** `fbb17c980` (the campaign's read-only `horizon/`
provider) has 186 hooks and is newer than `c9d474d` (108). The new pin contains
every hook `fbb17c980` has (no removals in the diff), so a hook **absent from
`fbb17c980` did not exist at `c9d474d`**; a hook present there may predate
`c9d474d`. Verdicts below use that conservative reading.

Class: **usable** = writable/cancel/replace/collect or a helper the bundle can
call; **hook-only** = listen-only; **absent** = none.

## Deferred features (13)

| # | Deferred (notes §Gaps) | Evidence at `83ad384f` | Class | Verdict | Next step |
|---|---|---|---|---|---|
| F1 | **Crop blocks and seeds** (cabbage/tomato/onion, wild crops, rice, rich soil/farmland) | A content block whose template is a vanilla crop already grows: `Class::Crop` → `crop_random_tick` (`horizon-world/src/level/tick.rs:334,851-865`), survival via `supports_crops` (`survive.rs:386`), bonemeal `Class::Crop → Kind::Crop` (`bonemeal.rs:69`). Own hooks if a custom growth rule is wanted: `native_random_tick` (`native_hooks.rs:1576`), `native_tick_block` (`:1687`), `native_is_valid_bonemeal_target` (`:1761`), `native_perform_bonemeal` (`:1809`), `native_moisture_change` (`:1251`), `native_block_form` (`:1288`); content `RandomTicks` (`horizon-mods/src/content.rs:124-152`); `native_set_block_property` | usable | **Realizable now, mostly by content.** Declare each crop on the vanilla crop template with the matching age range (`minecraft:beetroot`/`torchflower` for 0–3, `minecraft:wheat` for 0–7); growth, light check, support and bonemeal come from the template; loot tables keyed on `age` give seeds/produce. Rich soil/farmland via `native_moisture_change`. | Ship the crops/seeds slice first; use `moisture_change` for rich soil so it never dries. Prove growth (`random_tick_speed 0` differential) and harvest drops in the e2e. |
| F2 | **Cutting board + all `farmersdelight:cutting` recipes** | `native_use_item_on` (`:6482`, cancel) / `native_use_item` (`:6523`, cancel); block entity `native_block_entity` (`:2985`), `_ticks` (`:2996`), `_tick` (`:3122`), `_container` (`:3035`, one slot); the mod's own recipe type: `native_recipes_of_type("farmersdelight:cutting")` (`:9253`), `native_recipe_ingredient` (`:9262`), `native_recipe_result` (`:9303`); `native_loot_drops` (`:7458`) for the result | usable | **Realizable now.** Custom recipe types in `data/farmersdelight/recipe/*.json` are readable by the helpers (they read content-mod recipes, `native_hooks.rs:9243-9258`). | Add a one-slot BE whose `use_item_on` matches the held item against `recipes_of_type`, then drops/returns the result and wears the tool. Probe a knife on a cabbage and a wrong tool. |
| F3 | **Cooking pot + all `farmersdelight:cooking` recipes** | `native_block_entity_container` (`:3035`) + `block_entity_tick` (`:3122`) + `block_entity_fluids` (`:3050`) for the stew; `recipes_of_type("farmersdelight:cooking")` (`:9253`); `native_open_menu` (`native_menus.rs:124`) with a content menu type (`content.rs:402-420`); `native_campfire_start` (`:4826`) / `native_block_cook` (`:4830`, **campfire only**, listen) | usable | **Realizable now.** Carries the 23 deferred cooking recipes: a BE with N ingredient slots + a container menu + a per-tick timer, reading the recipe JSON through the helpers. | Add the pot BE with 6 slots, a content menu, and a tick that resolves `recipes_of_type`; then the deferred recipes become survival-obtainable. |
| F4 | **Skillet** (furnace-like cooking + serving) | Same BE hooks as F3; if the skillet uses a furnace template, the new `native_furnace_start_smelt`/`native_furnace_smelt`/`native_furnace_burn`/`native_furnace_extract` (all new since `fbb17c980`) make the vanilla path heap-visible | usable | **Realizable now.** Simplest is a one-slot `block_entity_container` that runs the cooking recipes on a timer (Skillet is a slower pot with a held item). | Build after F3; reuse the pot's recipe resolution with one input slot and its own use animation. |
| F5 | **Feast blocks** (roast chicken, stuffed pumpkin, …) + placeable pies/cheesecakes | `native_block_entity_container` (`:3035`) + `native_right_click_block` (`:6790`) / `native_use_item_on` (`:6482`) + `native_place_block` (`:6775`, listen); content `state_properties`/`default_state` (`content.rs:272-278`) for `servings`; `native_set_block_property` | usable | **Realizable now.** A feast is a BE-backed block with a `servings` state; a use takes one portion (give one item, decrement, air at 0). Pies/cheesecakes place a block item and swap to a block. | Add the feast block + servings state, then the placeable pies on `use_item_on` (place + consume the food item). |
| F6 | **`nourishment` effect** (on cooked rice, stews, soups, plated meals) | Content mob effects: `MobEffect` (`content.rs:364-375`, parsed at `:714-716`). Applying one from food: `on_consume_effects` `apply_effects` accepts a mod id (`horizon-items/src/items.rs:73-99,2183-2196`). Ticking: `native_apply_effect_tick` (`:7654`, **API 56**, read-only), `native_on_effect_added` (`:7675`), `native_mob_effect_added` (`:7687`). Hunger control: `native_food_level_changed` (`:7539`, **API 169**, change `to`/cancel) — but the doc says it is **not heard for the LAN host** (`:7531-7539`) and there is no saturation write | usable + caveat | **Partially realizable.** Declare the effect and apply it from the food; its tick can be heard; hunger loss can be cancelled for **guests**. The singleplayer host is not heard, and saturation cannot be written (no `food`/`saturation` helper). | Implement the effect + `apply_effects` on meals; cancel `food_level_changed` for guests; for the host use the in-crate `player_tick` food write (documented deviation) or file an issue-shaped request for a player food/saturation write. |
| F7 | **Non-food drinks** (milk bottle, hot cocoa, melon juice: clear effects / heal) | Content `minecraft:consumable` + `on_consume_effects` without a `food` component; `clear_all_effects` (`items.rs:101`), `apply_effects` with `instant_health` (`:87-99`); `native_use_item`/`use_item_on` (cancel) to own the click; `native_consume_effect` (`:3131`, listen) for a mod-specific kind (`:75-78` keeps a namespaced type whole for the hook) | usable | **Realizable now.** These are consumables, not foods; the component set is data and the three behaviours map onto existing consume-effect types. | Add the three drink items with `consumable` + `on_consume_effects`; probe milk clearing an effect and cocoa healing. |
| F8 | **Dog food feeding wolves / horse feed** | `native_interact_entity` (`:2516`, cancel/claimed, entity id + kind + held stack); `native_entity` refs (`native_entity_refs.rs:41`); `native_entity_merge_nbt` (`native_entity_data.rs:32`, **new**); `native_set_mob_health` (`native_mobs.rs`); `native_entity_tame` (`:5544`, listen) / `native_entity_breed` (`:5550`, listen) | usable | **Realizable now.** Intercept the click, heal/`InLove` via the NBT merge (or `set_mob_health`), consume one, claim. The custom entity types are vanilla wolves/horses, so no entity registration is needed. | Add the two interactions; probe a wolf at full health, a hurt wolf and a horse. |
| F9 | **Melon popsicle `clearFire`** | **Absent**: `ConsumeEffect` has only `ApplyStatus`/`RemoveStatus`/`ClearStatus`/`PlaySound` (`horizon-world/src/consume_effect.rs:5-12`), the JSON parser has no `clear_fire` kind (`items.rs:73-124`), and `native_entity_merge_nbt` explicitly refuses a player ("False for a player", `native_entity_data.rs:28-31`) | absent | **Blocked** (cosmetic: the fire is extinguished, the 3/1.2 food itself is already carried). | Keep the food values; issue request: "a `clear_fire` consume-effect type", or use an in-crate player fire write from `player_tick` (deviation). |
| F10 | **Compost values** | `minecraft:compostable` is an item component read from the stack (`horizon-items/src/items.rs:171-172,904`); `native_compost_item` (`:4834`, **new**, listen) | usable | **Realizable now** by content: replace the template apple's 30 % with the upstream layer count per food. | Add `minecraft:compostable: {layers: n}` to the 77 item declarations from the upstream `ComposterBlock` values; probe the composter. |
| F11 | **Recipe advancements** | Content `data/farmersdelight/advancement/*` (loaded) or `native_data_pack_files` (`:2377`, API 106); `native_player_advancement_criterion_grant` (**API 339**, new since `fbb17c980`), `native_player_advancement_done` (API 249), `native_recipe_discovered` (cancel) | usable | **Realizable now** (data-tier work). | Carry the upstream recipe advancements; they are pure data. |
| F12 | **Tooltips / client effect tooltips** | `native_tooltip` (`:6413`, collect), `native_tooltip_color` (`:6418`, **new since `fbb17c980`**), `native_tooltip_component` (replace), `native_screen_tooltip` (collect) | usable | **Realizable now** (client-side bundle). | Optional: add the food-effect tooltip and the `enabled` config gate (F13). |
| F13 | **Config** (`ENABLE_FOOD_EFFECT_TOOLTIP` etc.) | `native_register_configs` (`:2424`, **API 397**, new), `native_config_loading`/`reloading`/`unloading` (`:2432`), `native_config_dir` helper | usable (new) | **Realizable now — the gap closes.** API 397 is **new since `fbb17c980`**. | Register a `Client` config for the tooltip flag; the rest of the upstream config is client presentation. |

## Summary

- **Deferred: 13** (F1–F13).
- **Realizable now: 11** — F1, F2, F3, F4, F5, F7, F8, F10, F11, F12,
  F13. Of these, **F10 (`compost_item`), F13 (`register_configs`), F11
  (`player_advancement_criterion_grant`), F12 (`tooltip_color`) and the
  furnace hooks behind F4 are new since `fbb17c980`**, i.e. new versus
  `c9d474d`; F1–F3, F5, F7, F8 were scope gaps rather than pin gaps.
- **Partially realizable: 1** — F6 (`nourishment`): the effect and its tick
  are available, hunger loss can be cancelled for guests, but the singleplayer
  host is not heard and saturation has no write helper.
- **Still blocked: 1** — F9 (`clear_fire` consume effect / player NBT write).
- **Highest-value next step:** **F1, crops and seeds.** It is mostly content:
  the pin's `Class::Crop` template path already supplies growth, light,
  support and bonemeal, so a wheat/beetroot-template block plus age-keyed loot
  tables makes the raw crops farmable. That unlocks the food graph the mod's
  recipes depend on (and makes the cutting board/pot slices meaningful).
  Follow with F2+F3 (cutting board, cooking pot), which turn the 45 carried
  recipes into the full recipe set.
- The notes' separate "not exercised in-game" caveat (consume effects, bowl
  remainders, drink animation) is a test gap, not a pin gap; the previously
  missing `native_living_entity_use_item_finish` is present at this pin, so it
  can be exercised if a finish-time behaviour is ever needed.

No build, test or e2e was run for this audit; all claims above are source
reads at `83ad384f`.
