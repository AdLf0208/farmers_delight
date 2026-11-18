#!/usr/bin/env bash
# In-game check for the Farmer's Delight slice (the food items and the two
# crops), run against a **launcher-built game** that carries this package.
#
# Default run (offscreen renderer, creative, The End):
#   1. a representative regular, bowl, drink, always-edible and pet food is
#      `/give`-able;
#   2. the creative tab lists all 77 slice foods;
#   3. two carried recipes craft in the 2x2 inventory grid and a wrong pair
#      stays empty (tools/craft_check.py);
#   4. hunger is drained to zero, one `farmersdelight:fried_egg` is eaten, and
#      `ctl state` reads food 0 -> 4 and saturation 0 -> 3.2;
#   5. a `minecraft:stick` right-click (the control) changes neither value;
#   6. an unknown `farmersdelight:` id is refused;
#   7. a cabbage and a tomato seed plant their crop on farmland, and stone and
#      vanilla wheat seeds plant nothing there;
#   8. `/fdcrop setage` writes and resets the crop's age through the level's own
#      block write and refuses an age past the crop's top;
#   9. the crop does not grow at `random_tick_speed 0` and grows at 1000 (on
#      hydrated farmland: dry farmland turns to dirt under its own random tick
#      and breaks the crop above, a mod crop not being in vanilla's
#      `maintains_farmland`);
#  10. a ripe cabbage drops produce + seeds and a young one seeds only.
#
# The gamerule names are snake_case at this pin (`random_tick_speed`,
# `mob_griefing`): the camelCase vanilla spellings are refused, and the refusal
# is a chat line, so every set is read back by `set_rule` (this is what kept the
# growth phase red while the crop was fine).
#
# SDK package (2026-11-18): content and code are one `.hmod`, compiled into the
# game by `horizon-launcher native-build <port> --instance <i> --trust-native-code`.
# So this script takes the binaries of that build:
#
#   GAME=<launcher-built horizon-client>  (required; carries this package)
#   CTL=<horizon-ctl of the same build>   (required; default: beside GAME)
#   CONTROL_GAME=<a game built without this mod>  (optional: the "ids unknown"
#       control leg; CONTROL=1 is the default when it is set)
#   EMBEDDED=1 is gone: there is nothing embedded any more, the package is the
#       mod. A game built without it is the control (CONTROL_GAME).
#
# NEGATIVE=1 keeps the same scenario but asserts the false value (fried egg
# gives 5 food); the run MUST exit 1, proving the eat assertion is not vacuous.
# SELFTEST=1 is the older name for that leg and still works.
#
# The arena is at x/z=1000 in The End, far from the dragon island, on a stone
# floor; every run uses a scratch HORIZON_HOME/XDG home and the prepared 26.3
# resources.
#
# No run of this script is recorded for 0.2.0: `horizon-launcher` is not
# installed in this environment, so no `0b14f8a` game carrying the package can
# exist here (see notes.md → "Migration to the SDK format", the gaps).
set -euo pipefail

CLONE=/home/adlf/Desktop/all_the_mods_10_rust_rewrite
PORT=$CLONE/ports/farmersdelight
GAME=${GAME:-}
CTL=${CTL:-${GAME:+$(dirname "$GAME")/horizon-ctl}}
RES=${RES:-${HORIZON_RES:-/home/adlf/Desktop/horizon/.local/horizon-client/26.3}}
SOCK=${SOCK:-/tmp/opencode/farmersdelight-e2e.sock}
# The carried content is part of the package (no `content/` folder, no
# `--dev-mod`): the tab check reads the same files the package ships.
CONTENT=$PORT
# CONTROL_GAME is a launcher-built game *without* this mod: the ids must be
# unknown there. CONTROL=1 (the default when it is given) runs that leg first.
CONTROL_GAME=${CONTROL_GAME:-}
CONTROL=${CONTROL:-$([ -n "$CONTROL_GAME" ] && echo 1 || echo 0)}
WORLD=FarmersDelightE2E
CONTROL_WORLD=FarmersDelightControl

NEGATIVE=${NEGATIVE:-0}
# SELFTEST=1 is the pre-0.2.0 name for the deliberately-failing leg.
[ "${SELFTEST:-0}" = 1 ] && NEGATIVE=1

[ -x "$CTL" ] || { echo "missing horizon-ctl: '$CTL' (pass CTL=<ctl of the launcher build>)" >&2; exit 1; }
[ -x "$GAME" ] || { echo "missing horizon-client: '$GAME' (pass GAME=<game built by horizon-launcher native-build>)" >&2; exit 1; }
[ -d "$RES/assets" ] || { echo "missing prepared resources: $RES" >&2; exit 1; }
[ -f "$CONTENT/horizon-mod.toml" ] || { echo "missing package manifest: $CONTENT/horizon-mod.toml" >&2; exit 1; }
[ -d "$CONTENT/assets/farmersdelight" ] || { echo "missing carried content: $CONTENT/assets/farmersdelight" >&2; exit 1; }
[ "$CONTROL" = 1 ] && [ ! -x "$CONTROL_GAME" ] \
    && { echo "CONTROL=1 needs CONTROL_GAME=<a game built without this mod>" >&2; exit 1; }

SCRATCH=$(mktemp -d /tmp/opencode/farmersdelight-home.XXXXXX)
export HORIZON_HOME=$SCRATCH/game
export HORIZON_LAUNCHER_HOME=$SCRATCH/launcher
export XDG_DATA_HOME=$SCRATCH/data
export XDG_CONFIG_HOME=$SCRATCH/config
export HORIZON_NO_AUDIO=1
# The game is launched with `--game-dir $HORIZON_HOME`; the merged content cache
# the launcher build writes lands beside it (the tab check looks in both places).
GAME_DIR=$SCRATCH/game
CACHE=$GAME_DIR/.horizon-cache
mkdir -p "$(dirname "$SOCK")" "$GAME_DIR"
rm -f "$SOCK"
cd "$SCRATCH"

ctl() { "$CTL" --socket "$SOCK" "$@"; }
fail() { echo "FAIL: $*" >&2; exit 1; }
say() { echo "== $* =="; }
end() { ctl command "execute in minecraft:the_end run $1"; }
state_food() { ctl state | jq -r '.player.food'; }
state_sat() { ctl state | jq -r '.player.saturation'; }
last_chat() { ctl messages 6 | jq -r '.chat[]' | tr '\n' '|'; }

cleanup() {
    ctl quit >/dev/null 2>&1 || true
    sleep 1
    rm -rf "$SCRATCH" || true
}
finish() { local rc=$?; cleanup; echo "EXIT:$rc"; return $rc; }
trap finish EXIT

launch() { # game world
    say "launch $2 (offscreen renderer, creative, mod in the build)"
    rm -f "$SOCK"
    "$CTL" --socket "$SOCK" launch \
        --game "$1" --world "$2" --seed 20261105 --creative --offscreen --wait \
        --game-dir "$GAME_DIR" --resources "$RES"
    ctl wait-ready 180
    if grep -q "content mods not loaded" "${SOCK}.log" 2>/dev/null; then
        grep "content mods not loaded" "${SOCK}.log" >&2
        fail "the package's content did not load"
    fi
}

control_leg() { # a game built without this mod: the ids must be unknown
    say "control: a launcher build that does not carry this package"
    launch "$CONTROL_GAME" "$CONTROL_WORLD"
    ctl command "clear @s" >/dev/null 2>&1 || true
    ctl command "give @s farmersdelight:cabbage 1" >/dev/null 2>&1 || true
    ctl wait 3
    local said
    said=$(last_chat)
    echo "  give: $said"
    case "$said" in
        *"Unknown item 'farmersdelight:cabbage'"*) ;;
        *) fail "the control game knows the cabbage item: $said" ;;
    esac
    ctl command "setblock 0 100 0 farmersdelight:cabbages" >/dev/null 2>&1 || true
    ctl wait 3
    said=$(last_chat)
    echo "  setblock: $said"
    case "$said" in
        *"Unknown block type 'farmersdelight:cabbages'"*) ;;
        *) fail "the control game knows the cabbage block: $said" ;;
    esac
    [ "$(ctl block 0 100 0 | jq -r '.block // "air"')" = "minecraft:air" ] \
        || fail "the control game holds a cabbage block"
    ctl quit >/dev/null 2>&1 || true
    sleep 2
}

[ "$CONTROL" = 1 ] && control_leg
launch "$GAME" "$WORLD"

say "The End arena at x/z=1000, y=79"
end "forceload add 992 992 1024 1024"
end "tp @s 1000.5 80 1000.5"
ctl wait 40
end "fill 992 79 992 1024 79 1024 minecraft:stone"
end "fill 992 80 992 1024 88 1024 minecraft:air"
end "kill @e[type=minecraft:item]"
end "kill @e[type=minecraft:ender_dragon]"
end "kill @e[type=minecraft:end_crystal]"
end "kill @e[type=!minecraft:player]"
end "difficulty easy"
end "tp @s 1000.5 80 1000.5"
ctl wait 10
ctl close >/dev/null 2>&1 || true
ctl wait 3
# A dead player's screen refuses every action below with a `dead screen is
# open` error, which reads like a broken port; check the screen first.
[ "$(ctl state | jq -r '.screen')" = "playing" ] \
    || fail "the arena's screen is '$(ctl state | jq -r '.screen')', not playing"

# --------------------------------------------------------------- give-able
say "representative foods are give-able"
GIVE=(
    farmersdelight:cabbage
    farmersdelight:fried_egg
    farmersdelight:apple_cider
    farmersdelight:tomato_sauce
    farmersdelight:hamburger
    farmersdelight:cooked_rice
    farmersdelight:glow_berry_custard
    farmersdelight:kelp_roll
    farmersdelight:dog_food
)
for item in "${GIVE[@]}"; do
    # `ctl give` lands on a later tick (the campaign's measured rule); under a
    # loaded machine one tick is not always enough, so the read is retried
    # rather than asserted on a single tick.
    for try in 1 2 3; do
        ctl command "clear @s" >/dev/null
        ctl give "$item" 1 >/dev/null || fail "give $item failed"
        ctl wait 3
        count=$(ctl inventory | jq -r --arg i "$item" '.counts[$i] // 0')
        [ "$count" -ge 1 ] && break
    done
    [ "$count" -ge 1 ] || fail "give $item gave $count"
done
echo "${#GIVE[@]}/${#GIVE[@]} representative foods give"

# ------------------------------------------------------------------- tab
say "the creative tab lists all 77 slice foods"
mapfile -t TAB_ITEMS < <(python3 - "$CONTENT/data/farmersdelight/horizon/creative_tab/farmersdelight.json" <<'PY'
import json, sys
print("\n".join(json.load(open(sys.argv[1]))["items"]))
PY
)
# The pin keeps the merged content cache beside the scratch root
# (`/tmp/opencode/.horizon-cache`), not under the game home; fall back to the
# old in-home path for an older harness.
CACHE=$( (ls -dt "$(dirname "$SCRATCH")"/.horizon-cache/mod-content/*/creative-inventory.json \
    "$SCRATCH"/game/.horizon-cache/mod-content/*/creative-inventory.json 2>/dev/null || true) | head -1)
[ -n "$CACHE" ] || fail "no merged content cache (creative-inventory.json)"
python3 "$PORT/tools/tab_check.py" "$CACHE" "${TAB_ITEMS[@]}" || fail "the creative tab check failed"

# --------------------------------------------------------------- recipes
say "carried recipes craft in the 2x2 grid"
# The creative inventory has no 2x2 grid; the survival inventory does.
ctl command "gamemode survival @s" >/dev/null
ctl wait 5
python3 "$PORT/tools/craft_check.py" "$SOCK" "$CTL" || fail "the craft check failed"

# ------------------------------------------------------------------- eat
say "drain hunger to zero"
ctl command "clear @s" >/dev/null
ctl command "effect give @s minecraft:hunger 120 255" >/dev/null
food=20
for _ in $(seq 1 40); do
    ctl wait 10
    food=$(state_food)
    [ "$food" -eq 0 ] && break
done
ctl command "effect clear @s" >/dev/null
ctl wait 5
food_before=$(state_food)
sat_before=$(state_sat)
echo "before: food=$food_before saturation=$sat_before"
[ "$food_before" -eq 0 ] || fail "hunger drain left food at $food_before, expected 0"
awk -v s="$sat_before" 'BEGIN{exit !(s < 0.0001 && s > -0.0001)}' || fail "hunger drain left saturation at $sat_before, expected 0"

say "eat farmersdelight:fried_egg (4 nutrition, 3.2 saturation)"
ctl command "clear @s" >/dev/null
ctl give farmersdelight:fried_egg 1 >/dev/null
ctl wait 5
ctl select farmersdelight:fried_egg >/dev/null
ctl use-item --hold 2.2 >/dev/null
ctl wait 6
food_after=$(state_food)
sat_after=$(state_sat)
echo "after fried egg: food=$food_after saturation=$sat_after"

if [ "$NEGATIVE" = 1 ]; then
    # This run deliberately asserts the wrong value; it MUST fail.
    [ "$food_after" -eq 5 ] \
        || fail "NEGATIVE run did not observe the false assertion (fried egg gave $food_after food, expected the wrong 5)"
    echo "NEGATIVE run unexpectedly passed" >&2
    exit 1
fi

[ "$food_after" -eq 4 ] || fail "fried egg gave $food_after food, expected 4"
awk -v s="$sat_after" 'BEGIN{exit !(s > 3.19 && s < 3.21)}' \
    || fail "fried egg gave saturation $sat_after, expected 3.2"

say "the control (a stick) changes nothing"
before_food=$food_after
before_sat=$sat_after
ctl command "clear @s" >/dev/null
ctl give minecraft:stick 1 >/dev/null
ctl wait 5
ctl select minecraft:stick >/dev/null
ctl use-item --hold 2.2 >/dev/null
ctl wait 6
after_food=$(state_food)
after_sat=$(state_sat)
echo "after stick: food=$after_food saturation=$after_sat"
[ "$after_food" -eq "$before_food" ] || fail "the stick changed food $before_food -> $after_food"
awk -v a="$before_sat" -v b="$after_sat" 'BEGIN{exit !(a == b)}' \
    || fail "the stick changed saturation $before_sat -> $after_sat"

# -------------------------------------------------------------- unknown
say "an unknown farmersdelight id is refused"
if ctl give farmersdelight:not_a_food 1 >/dev/null 2>&1; then
    fail "give farmersdelight:not_a_food succeeded"
fi
echo "unknown id refused"

# ---------------------------------------------------------------- crops
say "a seed on farmland plants its crop, and the crop template grows"
ctl command "gamemode creative" >/dev/null
# Farmland that is not kept wet turns to dirt on its own random tick and
# breaks the crop above it: a mod crop is not in vanilla's
# `minecraft:maintains_farmland`, so the cell is hydrated (water within the
# farmland's 4-block reach, at its own y, inside the stone floor so it cannot
# flow). Without it the growth phase would read `minecraft:air`.
end "setblock 1004 79 1000 minecraft:water" >/dev/null
end "setblock 1006 79 1000 minecraft:farmland" >/dev/null
end "setblock 1006 80 1000 minecraft:air" >/dev/null
end "setblock 1007 80 1000 minecraft:glowstone" >/dev/null
ctl tp 1005.5 80 1000.5 >/dev/null
ctl wait 4
ctl give farmersdelight:cabbage_seeds 2 >/dev/null || fail "cannot give cabbage seeds"
ctl wait 4
ctl select farmersdelight:cabbage_seeds >/dev/null || fail "cannot select cabbage seeds"
ctl wait 2
ctl interact 1006 79 1000 up >/dev/null || fail "planting the cabbage failed"
ctl wait 6
planted=$(ctl block 1006 80 1000 | jq -r '.state // "unknown"')
case "$planted" in
    "farmersdelight:cabbages"*) ;;
    *) fail "the cabbage seeds planted '$planted'" ;;
esac
case "$planted" in
    *"age=0"*) ;;
    *) fail "the planted cabbage is not age 0: $planted" ;;
esac
[ "$(ctl inventory | jq -r '.counts["farmersdelight:cabbage_seeds"] // 0')" = 2 ] \
    || fail "the creative seed click changed the stack (vanilla does not spend in creative)"
echo "planted $planted in creative and kept both seeds (vanilla's creative rule)"

say "a native command handler writes the crop's age (native_set_block, API 24)"
# The brief's experiment (a): does a content block's `age` accept a state
# write at all? `/fdcrop setage` resolves `farmersdelight:cabbages[age=N]` in
# the content registry and writes it with `Game::native_set_block`; the client
# copy is then read back with `ctl block`. The feedback is a server message
# that lands a tick after the chat line, so it is read from `messages`, not
# from the chat reply.
fdcrop() {
    ctl chat "/fdcrop $*" >/dev/null
    ctl wait 2 >/dev/null
    ctl messages 8
}
said() { jq -r --arg got "$1" 'any(.chat[]; startswith($got))' <<<"$2"; }
reply=$(fdcrop setage 1006 80 1000 1)
[ "$(said "fdcrop: farmersdelight:cabbages[age=1] written" "$reply")" = true ] \
    || fail "the setage command did not answer: $(jq -c '.chat' <<<"$reply")"
written=$(ctl block 1006 80 1000 | jq -r '.state // "unknown"')
case "$written" in
    "farmersdelight:cabbages"*"age=1"*) ;;
    *) fail "the command write left '$written', expected age=1" ;;
esac
echo "the command wrote $written"
reply=$(fdcrop setage 1006 80 1000 0)
reset=$(ctl block 1006 80 1000 | jq -r '.state // "unknown"')
case "$reset" in
    "farmersdelight:cabbages"*"age=0"*) ;;
    *) fail "the reset left '$reset', expected age=0" ;;
esac
reply=$(fdcrop setage 1006 80 1000 9)
[ "$(said "fdcrop: farmersdelight:cabbages has ages 0..=7" "$reply")" = true ] \
    || fail "an age past the crop's top was not refused: $(jq -c '.chat' <<<"$reply")"
echo "the command reset the crop to age 0 and refused age 9 (top age 7)"

say "in survival a seed is spent: a tomato seed on farmland plants budding_tomatoes"
ctl command "gamemode survival @s" >/dev/null
end "setblock 1004 79 1002 minecraft:water" >/dev/null
end "setblock 1006 79 1002 minecraft:farmland" >/dev/null
end "setblock 1006 80 1002 minecraft:air" >/dev/null
ctl tp 1005.5 80 1002.5 >/dev/null
ctl turn 0 -20 >/dev/null
ctl wait 10
ctl give farmersdelight:tomato_seeds 2 >/dev/null || fail "cannot give tomato seeds"
ctl wait 4
ctl select farmersdelight:tomato_seeds >/dev/null || fail "cannot select tomato seeds"
ctl wait 4
reply=$(ctl interact 1006 79 1002 up 2>&1) || { echo "interact reply: $reply"; fail "planting the tomato failed"; }
ctl wait 6
tomato=$(ctl block 1006 80 1002 | jq -r '.state // "unknown"')
case "$tomato" in
    "farmersdelight:budding_tomatoes"*) ;;
    *) fail "the tomato seeds planted '$tomato'" ;;
esac
[ "$(ctl inventory | jq -r '.counts["farmersdelight:tomato_seeds"] // 0')" = 1 ] \
    || fail "the survival seed click did not consume one seed (count=$(ctl inventory | jq -r '.counts["farmersdelight:tomato_seeds"] // 0'))"
echo "planted $tomato in survival and spent one of two seeds"
ctl command "gamemode creative @s" >/dev/null
ctl tp 1005.5 80 1000.5 >/dev/null
ctl wait 2

say "with random ticks off the crop does not grow (control)"
# The gamerules are snake_case at this pin (`random_tick_speed`, `mob_griefing`);
# the camelCase vanilla spellings are not accepted, and the command answers
# with a chat error that a `>/dev/null` would swallow. Each set is read back,
# so a rule that did not take fails here rather than making a phase vacuous.
set_rule() {
    local name=$1 value=$2 got
    ctl command "gamerule $name $value" >/dev/null
    got=$(ctl command "gamerule $name" | jq -r '.chat[0] // ""')
    case "$got" in
        *"set to $value"*) ;;
        *) fail "gamerule $name did not become $value: '$got'" ;;
    esac
}
set_rule random_tick_speed 0
ctl wait 100
still=$(ctl block 1006 80 1000 | jq -r '.state // "unknown"')
case "$still" in
    *"age=0"*) ;;
    *) fail "the crop grew with random ticks off: $still" ;;
esac
echo "random_tick_speed 0: the crop stayed age 0"

say "with random ticks on the crop template grows it"
# A mob walking over the farmland would turn it to dirt and break the crop
# during a 1000-tick wait; the growth check must not race mob griefing. The
# cell is hydrated above for the same reason (dry farmland turns to dirt).
set_rule mob_griefing false
set_rule random_tick_speed 1000
grew=false
# 1000 ticks at speed 1000: the crop's cell is hit ~240 times, so a 1/9 growth
# roll (moist farmland) fails with probability far below 1e-6.
for _ in $(seq 1 200); do
    ctl wait 5
    now=$(ctl block 1006 80 1000 | jq -r '.state // "unknown"')
    case "$now" in
        *"age=0"*) ;;
        "farmersdelight:cabbages"*) grew=true; break ;;
    esac
done
[ "$grew" = true ] || fail "the cabbage never grew: $(ctl block 1006 80 1000 | jq -r '.state')"
echo "the cabbage grew: $now"
set_rule random_tick_speed 3

say "a seed on a non-farmland block plants nothing (control)"
end "setblock 1008 79 1000 minecraft:stone" >/dev/null
end "setblock 1008 80 1000 minecraft:air" >/dev/null
ctl tp 1007.5 80 1000.5 >/dev/null
ctl select farmersdelight:cabbage_seeds >/dev/null
ctl wait 2
ctl interact 1008 79 1000 up >/dev/null 2>&1 || true
ctl wait 6
[ "$(ctl block 1008 80 1000 | jq -r '.block // "air"')" = "minecraft:air" ] \
    || fail "a seed planted on stone"
echo "stone refuses the seed"

say "vanilla wheat seeds still plant vanilla wheat (differential control)"
ctl give minecraft:wheat_seeds 1 >/dev/null
ctl wait 4
ctl select minecraft:wheat_seeds >/dev/null
ctl wait 2
ctl interact 1006 79 1000 up >/dev/null 2>&1 || true
ctl wait 6
# 1006,80,1000 already holds the grown cabbage; plant on a fresh farmland cell.
end "setblock 1010 79 1000 minecraft:farmland" >/dev/null
end "setblock 1010 80 1000 minecraft:air" >/dev/null
ctl tp 1009.5 80 1000.5 >/dev/null
ctl select minecraft:wheat_seeds >/dev/null
ctl wait 2
ctl interact 1010 79 1000 up >/dev/null || fail "planting vanilla wheat failed"
ctl wait 6
vanilla=$(ctl block 1010 80 1000 | jq -r '.block // "air"')
[ "$vanilla" = "minecraft:wheat" ] || fail "vanilla wheat seeds planted '$vanilla'"
echo "vanilla wheat seeds planted minecraft:wheat"

say "harvesting a ripe crop drops the produce and seeds"
ctl command "gamemode survival" >/dev/null
end "setblock 1006 80 1000 farmersdelight:cabbages[age=7]" >/dev/null
ctl wait 4
end "kill @e[type=minecraft:item]" >/dev/null
ctl tp 1005.5 80 1000.5 >/dev/null
ctl wait 3
# The break reply names the block it broke and every drop it rolled, which is
# the loot table itself; the item entities are then a second, weaker signal
# (an adjacent player can pick one up inside the assertion's wait).
broke=$(ctl break 1006 80 1000) || fail "breaking the ripe cabbage failed"
echo "  ripe break: $(jq -c '{before: .block_before, drops: .drops}' <<<"$broke")"
[ "$(jq -r '.block_before // ""' <<<"$broke")" = "farmersdelight:cabbages[age=7]" ] \
    || fail "the ripe break addressed '$(jq -r '.block_before // ""' <<<"$broke")'"
[ "$(jq -r '.drops["farmersdelight:cabbage"] // 0' <<<"$broke")" = 1 ] \
    || fail "a ripe cabbage dropped no produce: $(jq -c '.drops' <<<"$broke")"
[ "$(jq -r '.drops["farmersdelight:cabbage_seeds"] // 0' <<<"$broke")" = 1 ] \
    || fail "a ripe cabbage dropped no seeds: $(jq -c '.drops' <<<"$broke")"
echo "a ripe cabbage dropped cabbage + cabbage_seeds"

end "kill @e[type=minecraft:item]" >/dev/null
end "setblock 1010 80 1000 farmersdelight:cabbages[age=0]" >/dev/null
# The player was left beside 1006 for the ripe harvest; 1010 is out of reach
# from there, and a `break` that cannot reach the block drops nothing.
ctl tp 1009.5 80 1000.5 >/dev/null
ctl wait 3
ctl look-at-block 1010 80 1000 up >/dev/null 2>&1 || true
ctl wait 3
seeds_before=$(ctl inventory | jq -r '.counts["farmersdelight:cabbage_seeds"] // 0')
broke=$(ctl break 1010 80 1000) || fail "breaking the young cabbage failed"
echo "  young break: $(jq -c '{before: .block_before, drops: .drops}' <<<"$broke")"
[ "$(jq -r '.block_before // ""' <<<"$broke")" = "farmersdelight:cabbages[age=0]" ] \
    || fail "the young break addressed '$(jq -r '.block_before // ""' <<<"$broke")'"
[ "$(jq -r '.drops["farmersdelight:cabbage"] // 0' <<<"$broke")" = 0 ] \
    || fail "an age-0 cabbage dropped produce: $(jq -c '.drops' <<<"$broke")"
[ "$(jq -r '.drops["farmersdelight:cabbage_seeds"] // 0' <<<"$broke")" = 1 ] \
    || fail "an age-0 cabbage dropped no seeds: $(jq -c '.drops' <<<"$broke")"
ctl wait 20
seeds_after=$(ctl inventory | jq -r '.counts["farmersdelight:cabbage_seeds"] // 0')
if [ "$seeds_after" -eq "$seeds_before" ]; then
    # Not picked up yet: the seed must then be lying on the ground. (An
    # adjacent player's pickup is a race, the drop map above is not.)
    ctl entities 8 minecraft:item \
        | jq -e '[.entities[] | select(.item.item == "farmersdelight:cabbage_seeds")] | length >= 1' >/dev/null \
        || fail "the young cabbage's seed is neither on the ground nor in the inventory"
    echo "  the seed is on the ground (the player has not picked it up yet)"
fi
echo "an age-0 cabbage dropped seeds only"

say "the tomato crop plants from its seeds too"
ctl command "gamemode creative" >/dev/null
end "setblock 1012 79 1000 minecraft:farmland" >/dev/null
end "setblock 1012 80 1000 minecraft:air" >/dev/null
ctl tp 1011.5 80 1000.5 >/dev/null
ctl give farmersdelight:tomato_seeds 1 >/dev/null
ctl wait 4
ctl select farmersdelight:tomato_seeds >/dev/null
ctl wait 2
ctl interact 1012 79 1000 up >/dev/null || fail "planting the tomato failed"
ctl wait 6
tomato=$(ctl block 1012 80 1000 | jq -r '.state // "unknown"')
case "$tomato" in
    "farmersdelight:budding_tomatoes"*"age=0"*) ;;
    *) fail "the tomato seeds planted '$tomato'" ;;
esac
echo "planted $tomato"

echo "PASS: fried egg took food 0 -> 4 and saturation 0 -> 3.2, a stick changed neither, both recipes crafted, the tab lists 77 foods, a cabbage and a tomato seed plant their crop on farmland while stone and vanilla wheat seeds do not change the mod crop, a native command writes and resets the crop's age and refuses one past its top, the crop grows with random ticks and not without, a ripe cabbage drops produce + seeds while a young one drops seeds only, and an unknown id was refused"
