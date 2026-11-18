//! Farmer's Delight for Horizon — slice 2 as one flat SDK package.
//!
//! Ported from Farmer's Delight 1.21.1-1.3.4 (`vectorwing/FarmersDelight`, MIT),
//! branch `1.21`, commit `dd8b9b0bfec916a2da88a6d64e3a37f4a02c3259`. The foods,
//! recipes, tags and the two crop blocks are **content** (`assets/`, `data/`); the
//! only code the slice needs is
//!
//! - the **seed click**: a crop block has no item of its own, so using its seed on
//!   the top face of farmland plants it, spends one seed and claims the click —
//!   vanilla's `BlockItem.place` rule, which the retired bundle injected at
//!   `ItemStack.useOn`;
//! - the **`/fdcrop` readout**: `setage` writes one of the two crops' `age` through
//!   the level's own block write, so the e2e can prove a content crop accepts a
//!   state write and refuses an age past its top. It is a harness diagnostic, not
//!   an upstream command.
//!
//! The numbers — every food's nutrition and saturation, the crop/seed table, the
//! command grammar — live in [`runtime`], which has no `mod_api` type and is
//! covered by `cargo test` alone.

pub mod runtime;

use mod_api::{
    self, BlockUpdateFlags, Command, CommandArgumentValue, Flow, Hooks, Host, HostSide,
    HostTransaction, InteractionResult, ItemPart, ItemPlace, ItemResource, LevelRef,
    RegisterCommandsEvent, Registrar, RegistrationContext, RunModCommandEvent, ServerHost, Text,
    UseOnEvent,
};

/// The two seed items that plant a crop (upstream's `CabbageSeedsItem`, the
/// tomato's seed item). The item→crop table itself is in [`runtime`].
const SEEDS: [&str; 2] = [
    "farmersdelight:cabbage_seeds",
    "farmersdelight:tomato_seeds",
];

/// The block a seed is used on (vanilla's `BlockItem` rule for a crop).
const FARMLAND: &str = "minecraft:farmland";
/// The empty cell a crop is planted into.
const AIR: &str = "minecraft:air";

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl Hooks for Mod {
    /// The seed click is a moment of *those two items*, so it is a Part attached to
    /// them, not a global hook that would run for every item in the game and filter
    /// by id — the same behaviour the retired `use_item_on` injection produced.
    fn register(&self, r: &mut dyn Registrar, _: &RegistrationContext) {
        for id in SEEDS {
            // Bound first: `item_part(r.item_key(id), …)` borrows `r` twice.
            let key = r.item_key(id);
            r.item_part(key, SeedPart);
        }
    }

    /// `/fdcrop setage <x> <y> <z> <age>`: one usage, the three coordinates as the
    /// dispatcher's `block_pos` so `~` resolves as every vanilla command's does.
    fn register_commands(&self, _: &mut dyn ServerHost, event: &mut RegisterCommandsEvent<'_>) {
        event
            .commands
            .push(Command::new("fdcrop setage <pos:block_pos> <age:integer>"));
    }

    fn run_mod_command(
        &self,
        host: &mut dyn ServerHost,
        event: &mut RunModCommandEvent<'_>,
    ) -> Flow {
        let command = event.command;
        if command.literals.first().map(String::as_str) != Some("fdcrop") {
            return Flow::Continue;
        }
        let Some(("setage", pos)) = command
            .literals
            .get(1)
            .map(String::as_str)
            .zip(block_pos(command))
        else {
            feedback(host, runtime::FD_CROP_USAGE);
            return Flow::Cancel;
        };
        let Some(CommandArgumentValue::Integer(age)) = command.argument("age") else {
            feedback(host, runtime::FD_CROP_USAGE);
            return Flow::Cancel;
        };
        set_age(host, command, pos, *age);
        Flow::Cancel
    }
}

/// The `pos` argument of a command, as the block it names.
fn block_pos(command: &mod_api::CommandInvocation) -> Option<[i32; 3]> {
    match command.argument("pos")? {
        CommandArgumentValue::BlockPos(pos) => Some(*pos),
        _ => None,
    }
}

/// The level a command executes in, after `execute in` (the console's is the
/// Overworld): the retired `Game::native_dimension`. `LevelRef::dimension` is the
/// level's own key, so the search needs no id string.
fn command_level(host: &dyn ServerHost, command: &mod_api::CommandInvocation) -> Option<LevelRef> {
    host.levels()
        .list_levels()
        .into_iter()
        .find(|level| level.dimension() == command.dimension)
}

/// The block at `pos` by id, `None` for a position this level does not hold.
fn block_id(host: &dyn Host, level: LevelRef, pos: [i32; 3]) -> Option<String> {
    level.block(host, pos).map(|block| block.id(host))
}

/// The seed click: one `ItemPart::use_on` on the two seed items, which runs where
/// `ItemStack.useOn` starts — after the clicked block let the click through, so a
/// farmland click is ours and a chest still opens.
///
/// The client half answers nothing (it has no authoritative write to make), and the
/// server half follows vanilla's `BlockItem` rule for a crop: the top face of
/// farmland, an empty cell above it, then one seed spent and the click used up.
#[derive(Clone, Copy, Debug, Default)]
struct SeedPart;

#[mod_api::horizon_part]
impl ItemPart for SeedPart {
    fn use_on(&self, host: HostSide<'_>, event: &mut UseOnEvent<'_>) -> Option<InteractionResult> {
        let HostSide::Server(host) = host else {
            return None;
        };
        // The top face only: `[0, 1, 0]` is vanilla's `Direction.UP`.
        if event.face != [0, 1, 0] {
            return None;
        }
        let item = event.stack.item().id(host);
        let crop = runtime::crop_for_seed(&item)?;
        let level = host.player(event.player)?.level(host)?;
        if block_id(host, level, event.pos).as_deref() != Some(FARMLAND) {
            return None;
        }
        let target = [event.pos[0], event.pos[1] + 1, event.pos[2]];
        if block_id(host, level, target).as_deref() != Some(AIR) {
            return None;
        }
        let state = host.block_state_id(crop)?;
        // `UPDATE_NEIGHBORS | UPDATE_CLIENTS`, the flags the retired `native_set_block`
        // write passed (a placed block tells its neighbours and the clients).
        if !matches!(
            level.set_block(
                host,
                target,
                state,
                BlockUpdateFlags::NEIGHBORS | BlockUpdateFlags::CLIENTS,
            ),
            Ok(true)
        ) {
            return None;
        }
        spend_one(host, event);
        Some(InteractionResult::SuccessServer)
    }
}

/// Take one seed out of the clicking hand. `ItemPlace::PlayerHand` is the engine's
/// own item move, so a creative hand gives the seed without losing it and the
/// player's client is sent the changed slot — the retired bundle needed a
/// `server_tick` queue and an explicit guest inventory re-send for that.
fn spend_one(host: &mut dyn ServerHost, event: &UseOnEvent<'_>) {
    let resource = ItemResource::of(event.stack);
    let place = ItemPlace::PlayerHand {
        player: event.player,
        hand: event.hand,
    };
    let _ = host.transaction(|host, tx| Ok::<u32, ()>(place.extract(host, tx, &resource, 1)));
}

/// `/fdcrop setage <x> <y> <z> <age>`: write one of our crops' `age` through the
/// level's block write (`Level.setBlock`) and answer with what changed.
///
/// The readout lines are the retired command's, byte for byte, so the e2e's
/// assertions still match. One deviation: the dispatcher hands `age` as a signed
/// integer (the old parser read a `u8` and answered the usage line for `-1` or
/// `256`), so an age outside the crop's range is refused by the crop's own
/// `has ages 0..=N` line instead.
fn set_age(
    host: &mut dyn ServerHost,
    command: &mod_api::CommandInvocation,
    pos: [i32; 3],
    age: i32,
) {
    let Some(level) = command_level(host, command) else {
        feedback(host, "fdcrop: no level to read");
        return;
    };
    let Some((crop, max_age)) = block_id(host, level, pos)
        .and_then(|id| runtime::crop_max_age(&id).map(|max_age| (id, max_age)))
    else {
        feedback(
            host,
            format!("fdcrop: no crop at [{}, {}, {}]", pos[0], pos[1], pos[2]),
        );
        return;
    };
    if age < 0 || age > i32::from(max_age) {
        feedback(host, format!("fdcrop: {crop} has ages 0..={max_age}"));
        return;
    }
    let Some(state) = host.block_state_id(&format!("{crop}[age={age}]")) else {
        feedback(
            host,
            format!("fdcrop: {crop}[age={age}] is not a registered state"),
        );
        return;
    };
    // Flag 3 again: notify neighbours and send the new state to the clients, as the
    // retired `native_set_block` call did.
    let changed = matches!(
        level.set_block(
            host,
            pos,
            state,
            BlockUpdateFlags::NEIGHBORS | BlockUpdateFlags::CLIENTS,
        ),
        Ok(true)
    );
    feedback(
        host,
        format!(
            "fdcrop: {crop}[age={age}] {}",
            if changed { "written" } else { "unchanged" }
        ),
    );
}

/// Answer through the command's own feedback (the console, a command block and a
/// player's chat all take it; the retired `native_feedback` did the same).
fn feedback(host: &mut dyn ServerHost, line: impl Into<String>) {
    let _ = host.commands().feedback(Text::literal(line.into()));
}
