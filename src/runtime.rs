// Farmer's Delight for Horizon — the slice-1 food contract (pure).
//
// Ported from Farmer's Delight 1.21.1-1.3.4 (`vectorwing/FarmersDelight`,
// branch `1.21`, commit `dd8b9b0bfec916a2da88a6d64e3a37f4a02c3259`) for
// Minecraft Java 26.3 on Horizon. MIT upstream; this port is local-only until
// the maintainer approves publication (ARCHITECTURE.md §8).
//
// Slice 1 is the food items and their nutrition: the 77 edible items the mod
// registers, their nutrition and absolute saturation (the 26.3 component's
// unit), stack size, use remainder, consume time and the vanilla status
// effects they apply. The cutting board, cooking pot, skillet and the crop
// blocks are deferred. This module is the pure contract the `.hmod` in
// `../content` is generated from and the in-game e2e checks.
//
// The whole module is test-only: the copy injected into a game build is empty,
// so its table has no dead-code warnings, while `cargo test` pins the content
// the generator writes (the Dyenamics/Productive Trees pattern).

/// One slice-1 food item.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(not(test), allow(dead_code))]
pub struct Food {
    /// The full item id, `farmersdelight:<name>`.
    pub id: &'static str,
    /// Hunger points restored.
    pub nutrition: u32,
    /// Absolute saturation restored (26.3 `minecraft:food.saturation`).
    pub saturation: f32,
    /// The vanilla item whose components the declaration copies.
    pub template: &'static str,
    /// Maximum stack size.
    pub stack: u32,
    /// Use duration in seconds.
    pub consume_seconds: f32,
    /// Whether it can be eaten at full hunger.
    pub always_edible: bool,
    /// What the stack leaves behind when eaten.
    pub remainder: Option<&'static str>,
    /// Whether the use animation is `drink` rather than `eat`.
    pub drink: bool,
    /// `(effect id, duration ticks, amplifier, probability)`.
    pub effects: &'static [(&'static str, u32, u32, f32)],
}

/// The 77 edible items, in upstream `ModItems` registration order (which
/// is also the creative-tab order).
#[cfg_attr(not(test), allow(dead_code))]
pub const FOODS: [Food; 77] = [
    Food {
        id: "farmersdelight:cabbage",
        nutrition: 2,
        saturation: 1.60,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:tomato",
        nutrition: 1,
        saturation: 0.60,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:onion",
        nutrition: 2,
        saturation: 1.60,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:fried_egg",
        nutrition: 4,
        saturation: 3.20,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:apple_cider",
        nutrition: 0,
        saturation: 0.00,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: true,
        remainder: Some("minecraft:glass_bottle"),
        drink: true,
        effects: &[("minecraft:absorption", 1200, 0, 1.0)],
    },
    Food {
        id: "farmersdelight:tomato_sauce",
        nutrition: 4,
        saturation: 3.20,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:wheat_dough",
        nutrition: 2,
        saturation: 1.20,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[("minecraft:hunger", 600, 0, 0.3)],
    },
    Food {
        id: "farmersdelight:raw_pasta",
        nutrition: 2,
        saturation: 1.20,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[("minecraft:hunger", 600, 0, 0.3)],
    },
    Food {
        id: "farmersdelight:pumpkin_slice",
        nutrition: 3,
        saturation: 1.80,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:cabbage_leaf",
        nutrition: 1,
        saturation: 0.80,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 0.8,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:minced_beef",
        nutrition: 2,
        saturation: 1.20,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 0.8,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:beef_patty",
        nutrition: 4,
        saturation: 6.40,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 0.8,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:chicken_cuts",
        nutrition: 1,
        saturation: 0.60,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 0.8,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[("minecraft:hunger", 600, 0, 0.3)],
    },
    Food {
        id: "farmersdelight:cooked_chicken_cuts",
        nutrition: 3,
        saturation: 3.60,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 0.8,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:bacon",
        nutrition: 2,
        saturation: 1.20,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 0.8,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:cooked_bacon",
        nutrition: 4,
        saturation: 6.40,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 0.8,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:cod_slice",
        nutrition: 1,
        saturation: 0.20,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 0.8,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:cooked_cod_slice",
        nutrition: 3,
        saturation: 3.00,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 0.8,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:salmon_slice",
        nutrition: 1,
        saturation: 0.20,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 0.8,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:cooked_salmon_slice",
        nutrition: 3,
        saturation: 4.80,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 0.8,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:mutton_chops",
        nutrition: 1,
        saturation: 0.60,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 0.8,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:cooked_mutton_chops",
        nutrition: 3,
        saturation: 4.80,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 0.8,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:ham",
        nutrition: 5,
        saturation: 3.00,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:smoked_ham",
        nutrition: 10,
        saturation: 16.00,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:pie_crust",
        nutrition: 2,
        saturation: 0.80,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:cake_slice",
        nutrition: 2,
        saturation: 0.40,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 0.8,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[("minecraft:speed", 400, 0, 1.0)],
    },
    Food {
        id: "farmersdelight:apple_pie_slice",
        nutrition: 3,
        saturation: 1.80,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 0.8,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[("minecraft:speed", 600, 0, 1.0)],
    },
    Food {
        id: "farmersdelight:sweet_berry_cheesecake_slice",
        nutrition: 3,
        saturation: 1.80,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 0.8,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[("minecraft:speed", 600, 0, 1.0)],
    },
    Food {
        id: "farmersdelight:chocolate_pie_slice",
        nutrition: 3,
        saturation: 1.80,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 0.8,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[("minecraft:speed", 600, 0, 1.0)],
    },
    Food {
        id: "farmersdelight:pumpkin_pie_slice",
        nutrition: 3,
        saturation: 1.80,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 0.8,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[("minecraft:speed", 600, 0, 1.0)],
    },
    Food {
        id: "farmersdelight:sweet_berry_cookie",
        nutrition: 2,
        saturation: 0.40,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 0.8,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:honey_cookie",
        nutrition: 2,
        saturation: 0.40,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 0.8,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:melon_popsicle",
        nutrition: 3,
        saturation: 1.20,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 0.8,
        always_edible: true,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:glow_berry_custard",
        nutrition: 7,
        saturation: 8.40,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: true,
        remainder: Some("minecraft:glass_bottle"),
        drink: false,
        effects: &[("minecraft:glowing", 100, 0, 1.0)],
    },
    Food {
        id: "farmersdelight:fruit_salad",
        nutrition: 6,
        saturation: 7.20,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[("minecraft:regeneration", 100, 0, 1.0)],
    },
    Food {
        id: "farmersdelight:mixed_salad",
        nutrition: 6,
        saturation: 7.20,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[("minecraft:regeneration", 100, 0, 1.0)],
    },
    Food {
        id: "farmersdelight:nether_salad",
        nutrition: 5,
        saturation: 4.00,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[("minecraft:nausea", 240, 0, 0.3)],
    },
    Food {
        id: "farmersdelight:barbecue_stick",
        nutrition: 8,
        saturation: 14.40,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:egg_sandwich",
        nutrition: 8,
        saturation: 12.80,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:chicken_sandwich",
        nutrition: 10,
        saturation: 16.00,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:hamburger",
        nutrition: 11,
        saturation: 17.60,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:bacon_sandwich",
        nutrition: 10,
        saturation: 16.00,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:mutton_wrap",
        nutrition: 10,
        saturation: 16.00,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:dumplings",
        nutrition: 8,
        saturation: 12.80,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:stuffed_potato",
        nutrition: 10,
        saturation: 14.00,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:cabbage_rolls",
        nutrition: 5,
        saturation: 5.00,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:salmon_roll",
        nutrition: 7,
        saturation: 8.40,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:cod_roll",
        nutrition: 7,
        saturation: 8.40,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:kelp_roll",
        nutrition: 12,
        saturation: 12.00,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 2.4,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:kelp_roll_slice",
        nutrition: 6,
        saturation: 6.00,
        template: "minecraft:apple",
        stack: 64,
        consume_seconds: 0.8,
        always_edible: false,
        remainder: None,
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:cooked_rice",
        nutrition: 6,
        saturation: 4.80,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:bone_broth",
        nutrition: 8,
        saturation: 11.20,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: true,
        effects: &[],
    },
    Food {
        id: "farmersdelight:beef_stew",
        nutrition: 12,
        saturation: 19.20,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:chicken_soup",
        nutrition: 12,
        saturation: 19.20,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:vegetable_soup",
        nutrition: 12,
        saturation: 19.20,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:fish_stew",
        nutrition: 12,
        saturation: 19.20,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:fried_rice",
        nutrition: 12,
        saturation: 19.20,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:pumpkin_soup",
        nutrition: 14,
        saturation: 21.00,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:baked_cod_stew",
        nutrition: 14,
        saturation: 21.00,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:noodle_soup",
        nutrition: 14,
        saturation: 21.00,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:onion_soup",
        nutrition: 12,
        saturation: 19.20,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:bacon_and_eggs",
        nutrition: 10,
        saturation: 12.00,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:pasta_with_meatballs",
        nutrition: 12,
        saturation: 19.20,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:pasta_with_mutton_chop",
        nutrition: 12,
        saturation: 19.20,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:mushroom_rice",
        nutrition: 12,
        saturation: 19.20,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:roasted_mutton_chops",
        nutrition: 14,
        saturation: 21.00,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:vegetable_noodles",
        nutrition: 14,
        saturation: 21.00,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:steak_and_potatoes",
        nutrition: 12,
        saturation: 19.20,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:ratatouille",
        nutrition: 10,
        saturation: 12.00,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:squid_ink_pasta",
        nutrition: 14,
        saturation: 21.00,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:grilled_salmon",
        nutrition: 14,
        saturation: 21.00,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:roast_chicken",
        nutrition: 14,
        saturation: 21.00,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:stuffed_pumpkin",
        nutrition: 14,
        saturation: 21.00,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:honey_glazed_ham",
        nutrition: 14,
        saturation: 21.00,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:shepherds_pie",
        nutrition: 14,
        saturation: 21.00,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:gleaming_salad",
        nutrition: 14,
        saturation: 21.00,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
    Food {
        id: "farmersdelight:dog_food",
        nutrition: 4,
        saturation: 1.60,
        template: "minecraft:apple",
        stack: 16,
        consume_seconds: 1.6,
        always_edible: false,
        remainder: Some("minecraft:bowl"),
        drink: false,
        effects: &[],
    },
];

#[cfg(test)]
fn food(id: &str) -> &'static Food {
    FOODS
        .iter()
        .find(|food| food.id == id)
        .unwrap_or_else(|| panic!("missing food {id}"))
}

/// The crop block a seed item plants. The crop blocks have no item of
/// their own (`item: false`), so planting is the native half's job.
pub fn crop_for_seed(item: &str) -> Option<&'static str> {
    match item {
        "farmersdelight:cabbage_seeds" => Some("farmersdelight:cabbages"),
        "farmersdelight:tomato_seeds" => Some("farmersdelight:budding_tomatoes"),
        _ => None,
    }
}

/// The two crops this slice ships, with the template that gives them
/// growth, light, support and bone meal: wheat (age 0..=7) and
/// beetroots (age 0..=3).
#[cfg_attr(not(test), allow(dead_code))]
pub const CROPS: [(&str, &str, u8); 2] = [
    ("farmersdelight:cabbages", "minecraft:wheat", 7),
    ("farmersdelight:budding_tomatoes", "minecraft:beetroots", 3),
];

/// The highest `age` state of a crop this slice ships, `None` for any other
/// block (a vanilla crop included).
#[must_use]
pub fn crop_max_age(block: &str) -> Option<u8> {
    CROPS
        .iter()
        .find(|(crop, _, _)| *crop == block)
        .map(|(_, _, age)| *age)
}

/// One `/fdcrop setage` request: a position and the `age` to write there.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AgeWrite {
    pub pos: [i32; 3],
    pub age: u8,
}

/// The line `/fdcrop …` (without its slash), once it is this mod's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CropCommand {
    /// `fdcrop setage <x> <y> <z> <age>`: write one of our crops' age.
    SetAge(AgeWrite),
    /// One of ours with the wrong shape; the line says what to type.
    Usage(&'static str),
}

/// What `/fdcrop` answers for a line it cannot run.
pub const FD_CROP_USAGE: &str = "usage: /fdcrop setage <x> <y> <z> <age>";

/// Parses a `/fdcrop` line (`command`, without its slash): `None` for another
/// mod's or the game's command, `Some` once the line is ours.
///
/// The parser is a syntax check only: whether the addressed block is one of
/// our crops, and whether the age is one that crop has, is decided where the
/// block is known (`crop_max_age`).
#[must_use]
pub fn parse_command(command: &str) -> Option<CropCommand> {
    let mut words = command.split_whitespace();
    if words.next() != Some("fdcrop") {
        return None;
    }
    if words.next() != Some("setage") {
        return Some(CropCommand::Usage(FD_CROP_USAGE));
    }
    let mut number = || words.next().and_then(|word| word.parse::<i32>().ok());
    let [Some(x), Some(y), Some(z)] = [number(), number(), number()] else {
        return Some(CropCommand::Usage(FD_CROP_USAGE));
    };
    let Some(age) = words.next().and_then(|word| word.parse::<u8>().ok()) else {
        return Some(CropCommand::Usage(FD_CROP_USAGE));
    };
    if words.next().is_some() {
        return Some(CropCommand::Usage(FD_CROP_USAGE));
    }
    Some(CropCommand::SetAge(AgeWrite {
        pos: [x, y, z],
        age,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slice_has_every_food_and_unique_ids() {
        assert_eq!(FOODS.len(), 77, "the slice carries all 77 edible items");
        let mut ids: Vec<&str> = FOODS.iter().map(|food| food.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), FOODS.len(), "food ids are unique");
        for food in &FOODS {
            assert!(
                food.id.starts_with("farmersdelight:"),
                "{} is namespaced",
                food.id
            );
            assert!(
                food.id.strip_prefix("farmersdelight:").is_some_and(|name| {
                    !name.is_empty()
                        && name
                            .bytes()
                            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
                }),
                "{} is a valid path",
                food.id
            );
        }
    }

    #[test]
    fn representative_nutrition_and_saturation() {
        // 1.21.1 `saturationModifier(m)` means nutrition * m * 2; 26.3 stores
        // the absolute value, so the table is the converted number.
        for (id, nutrition, saturation) in [
            ("farmersdelight:fried_egg", 4, 3.2),
            ("farmersdelight:hamburger", 11, 17.6),
            ("farmersdelight:cooked_rice", 6, 4.8),
            ("farmersdelight:beef_stew", 12, 19.2),
            ("farmersdelight:pumpkin_soup", 14, 21.0),
            ("farmersdelight:smoked_ham", 10, 16.0),
            ("farmersdelight:tomato", 1, 0.6),
        ] {
            let food = food(id);
            assert_eq!(food.nutrition, nutrition, "{id} nutrition");
            assert!(
                (food.saturation - saturation).abs() < 1e-4,
                "{id} saturation is {}, expected {saturation}",
                food.saturation
            );
        }
    }

    #[test]
    fn kelp_roll_is_the_direct_constructor_outlier() {
        // `FoodValues.KELP_ROLL` bypasses the builder: nutrition 12 and the
        // absolute saturation 12, not 12 * 0.5 * 2.
        let roll = food("farmersdelight:kelp_roll");
        assert_eq!(roll.nutrition, 12);
        assert!((roll.saturation - 12.0).abs() < 1e-4);
        assert!((roll.consume_seconds - 2.4).abs() < 1e-4);
        assert_eq!(roll.stack, 64);
        assert_eq!(roll.remainder, None);
    }

    #[test]
    fn apple_cider_is_a_zero_nutrition_always_edible_drink() {
        let cider = food("farmersdelight:apple_cider");
        assert_eq!(cider.nutrition, 0);
        assert!((cider.saturation - 0.0).abs() < 1e-4);
        assert!(cider.always_edible);
        assert!(cider.drink);
        assert_eq!(cider.stack, 16);
        assert_eq!(cider.remainder, Some("minecraft:glass_bottle"));
        assert_eq!(cider.effects, &[("minecraft:absorption", 1200, 0, 1.0)]);
    }

    #[test]
    fn bowl_foods_stack_sixteen_and_return_the_bowl() {
        let bowls = [
            "farmersdelight:fruit_salad",
            "farmersdelight:mixed_salad",
            "farmersdelight:nether_salad",
            "farmersdelight:cooked_rice",
            "farmersdelight:bone_broth",
            "farmersdelight:beef_stew",
            "farmersdelight:chicken_soup",
            "farmersdelight:vegetable_soup",
            "farmersdelight:fish_stew",
            "farmersdelight:fried_rice",
            "farmersdelight:pumpkin_soup",
            "farmersdelight:baked_cod_stew",
            "farmersdelight:noodle_soup",
            "farmersdelight:onion_soup",
            "farmersdelight:bacon_and_eggs",
            "farmersdelight:pasta_with_meatballs",
            "farmersdelight:pasta_with_mutton_chop",
            "farmersdelight:mushroom_rice",
            "farmersdelight:roasted_mutton_chops",
            "farmersdelight:vegetable_noodles",
            "farmersdelight:steak_and_potatoes",
            "farmersdelight:ratatouille",
            "farmersdelight:squid_ink_pasta",
            "farmersdelight:grilled_salmon",
            "farmersdelight:roast_chicken",
            "farmersdelight:stuffed_pumpkin",
            "farmersdelight:honey_glazed_ham",
            "farmersdelight:shepherds_pie",
            "farmersdelight:gleaming_salad",
            "farmersdelight:dog_food",
        ];
        assert_eq!(bowls.len(), 30);
        for id in bowls {
            let food = food(id);
            assert_eq!(food.stack, 16, "{id} stacks to 16");
            assert_eq!(
                food.remainder,
                Some("minecraft:bowl"),
                "{id} returns a bowl"
            );
        }
        // Tomato sauce is a bowl-remainder item at the default stack size.
        let sauce = food("farmersdelight:tomato_sauce");
        assert_eq!(sauce.stack, 64);
        assert_eq!(sauce.remainder, Some("minecraft:bowl"));
    }

    #[test]
    fn drinks_return_a_glass_bottle() {
        for id in [
            "farmersdelight:apple_cider",
            "farmersdelight:glow_berry_custard",
        ] {
            let food = food(id);
            assert_eq!(food.stack, 16, "{id} stacks to 16");
            assert_eq!(
                food.remainder,
                Some("minecraft:glass_bottle"),
                "{id} returns a glass bottle"
            );
        }
        // Bone broth is a bowl food drunk with the drink animation.
        let broth = food("farmersdelight:bone_broth");
        assert!(broth.drink);
        assert_eq!(broth.remainder, Some("minecraft:bowl"));
    }

    #[test]
    fn fast_foods_use_point_eight_seconds() {
        let fast = [
            "farmersdelight:cabbage_leaf",
            "farmersdelight:minced_beef",
            "farmersdelight:beef_patty",
            "farmersdelight:chicken_cuts",
            "farmersdelight:cooked_chicken_cuts",
            "farmersdelight:bacon",
            "farmersdelight:cooked_bacon",
            "farmersdelight:cod_slice",
            "farmersdelight:cooked_cod_slice",
            "farmersdelight:salmon_slice",
            "farmersdelight:cooked_salmon_slice",
            "farmersdelight:mutton_chops",
            "farmersdelight:cooked_mutton_chops",
            "farmersdelight:cake_slice",
            "farmersdelight:apple_pie_slice",
            "farmersdelight:sweet_berry_cheesecake_slice",
            "farmersdelight:chocolate_pie_slice",
            "farmersdelight:pumpkin_pie_slice",
            "farmersdelight:sweet_berry_cookie",
            "farmersdelight:honey_cookie",
            "farmersdelight:melon_popsicle",
            "farmersdelight:kelp_roll_slice",
        ];
        assert_eq!(fast.len(), 22);
        for id in fast {
            assert!(
                (food(id).consume_seconds - 0.8).abs() < 1e-4,
                "{id} is fast (0.8 s)"
            );
        }
        // Everything else eats in 1.6 s, except the kelp roll.
        for food in &FOODS {
            if food.id == "farmersdelight:kelp_roll" || fast.contains(&food.id) {
                continue;
            }
            assert!(
                (food.consume_seconds - 1.6).abs() < 1e-4,
                "{} is a normal 1.6 s use",
                food.id
            );
        }
    }

    #[test]
    fn only_three_foods_are_always_edible() {
        let mut always: Vec<&str> = FOODS
            .iter()
            .filter(|food| food.always_edible)
            .map(|food| food.id)
            .collect();
        always.sort_unstable();
        assert_eq!(
            always,
            [
                "farmersdelight:apple_cider",
                "farmersdelight:glow_berry_custard",
                "farmersdelight:melon_popsicle",
            ]
        );
    }

    #[test]
    fn vanilla_effects_are_carried() {
        for id in [
            "farmersdelight:wheat_dough",
            "farmersdelight:raw_pasta",
            "farmersdelight:chicken_cuts",
        ] {
            assert_eq!(
                food(id).effects,
                &[("minecraft:hunger", 600, 0, 0.3)],
                "{id} has the 30 % hunger effect"
            );
        }
        assert_eq!(
            food("farmersdelight:cake_slice").effects,
            &[("minecraft:speed", 400, 0, 1.0)]
        );
        for id in [
            "farmersdelight:apple_pie_slice",
            "farmersdelight:sweet_berry_cheesecake_slice",
            "farmersdelight:chocolate_pie_slice",
            "farmersdelight:pumpkin_pie_slice",
        ] {
            assert_eq!(
                food(id).effects,
                &[("minecraft:speed", 600, 0, 1.0)],
                "{id} has the speed effect"
            );
        }
        for id in ["farmersdelight:fruit_salad", "farmersdelight:mixed_salad"] {
            assert_eq!(food(id).effects, &[("minecraft:regeneration", 100, 0, 1.0)]);
        }
        assert_eq!(
            food("farmersdelight:nether_salad").effects,
            &[("minecraft:nausea", 240, 0, 0.3)]
        );
        assert_eq!(
            food("farmersdelight:glow_berry_custard").effects,
            &[("minecraft:glowing", 100, 0, 1.0)]
        );
    }

    #[test]
    fn nourishment_is_deferred() {
        // The mod's own `nourishment` effect needs a registered mob effect and
        // a hunger-tick hook; slice 1 carries the nutrition only.
        for food in &FOODS {
            for (id, ..) in food.effects {
                assert!(
                    !id.starts_with("farmersdelight:"),
                    "{} still names the deferred nourishment effect",
                    food.id
                );
            }
        }
        for id in [
            "farmersdelight:cooked_rice",
            "farmersdelight:beef_stew",
            "farmersdelight:roast_chicken",
        ] {
            assert!(
                food(id).effects.is_empty(),
                "{id} carries no slice-1 effect"
            );
        }
    }

    #[test]
    fn all_items_copy_the_apple_template() {
        // `minecraft:apple` supplies `consumable`, `food`, `use_effects` and
        // the 64 stack; each declaration overrides what differs.
        for food in &FOODS {
            assert_eq!(food.template, "minecraft:apple", "{}", food.id);
        }
    }

    #[test]
    fn creative_tab_order_matches_upstream_registration() {
        assert_eq!(FOODS[0].id, "farmersdelight:cabbage");
        assert_eq!(FOODS[2].id, "farmersdelight:onion");
        assert_eq!(FOODS[3].id, "farmersdelight:fried_egg");
        assert_eq!(FOODS[4].id, "farmersdelight:apple_cider");
        assert_eq!(FOODS[28].id, "farmersdelight:chocolate_pie_slice");
        assert_eq!(FOODS[34].id, "farmersdelight:fruit_salad");
        assert_eq!(FOODS[50].id, "farmersdelight:cooked_rice");
        assert_eq!(FOODS[62].id, "farmersdelight:pasta_with_meatballs");
        assert_eq!(FOODS[76].id, "farmersdelight:dog_food");
    }

    #[test]
    fn nutrition_and_saturation_are_sane() {
        for food in &FOODS {
            assert!(food.nutrition <= 20, "{} nutrition", food.id);
            assert!(food.saturation.is_finite(), "{} saturation", food.id);
            assert!(food.saturation >= 0.0, "{} saturation", food.id);
            assert!(food.stack == 16 || food.stack == 64, "{} stack", food.id);
        }
    }

    #[test]
    fn only_the_two_seeds_plant_a_crop() {
        assert_eq!(
            crop_for_seed("farmersdelight:cabbage_seeds"),
            Some("farmersdelight:cabbages")
        );
        assert_eq!(
            crop_for_seed("farmersdelight:tomato_seeds"),
            Some("farmersdelight:budding_tomatoes")
        );
        assert_eq!(crop_for_seed("minecraft:wheat_seeds"), None);
        assert_eq!(crop_for_seed("farmersdelight:onion"), None);
        assert_eq!(crop_for_seed("farmersdelight:cabbage"), None);
    }

    #[test]
    fn the_crops_use_vanilla_crop_templates() {
        for (crop, template, age) in CROPS {
            assert!(crop.starts_with("farmersdelight:"));
            assert!(template.starts_with("minecraft:"));
            assert!(age == 3 || age == 7, "{crop} max age {age}");
        }
    }

    #[test]
    fn crop_max_age_is_the_declared_top_state() {
        assert_eq!(crop_max_age("farmersdelight:cabbages"), Some(7));
        assert_eq!(crop_max_age("farmersdelight:budding_tomatoes"), Some(3));
        assert_eq!(crop_max_age("minecraft:wheat"), None);
        assert_eq!(crop_max_age("farmersdelight:cabbage"), None);
    }

    #[test]
    fn setage_parses_its_four_arguments() {
        assert_eq!(
            parse_command("fdcrop setage 1006 80 1000 1"),
            Some(CropCommand::SetAge(AgeWrite {
                pos: [1006, 80, 1000],
                age: 1,
            }))
        );
        assert_eq!(
            parse_command("fdcrop setage -12 70 -3 7"),
            Some(CropCommand::SetAge(AgeWrite {
                pos: [-12, 70, -3],
                age: 7,
            }))
        );
    }

    #[test]
    fn another_mods_command_is_not_ours() {
        assert_eq!(parse_command("say hello"), None);
        assert_eq!(parse_command("fdcropper setage 1 2 3 4"), None);
        assert_eq!(parse_command(""), None);
    }

    #[test]
    fn a_malformed_setage_answers_with_its_usage() {
        // Ours, but not a form we run: claimed, with the usage line.
        for line in [
            "fdcrop",
            "fdcrop setage",
            "fdcrop setage 1 2 3",
            "fdcrop setage 1 2 3 4 5",
            "fdcrop setage 1 2 3 abc",
            "fdcrop setage 1.5 2 3 4",
            "fdcrop nope 1 2 3 4",
        ] {
            assert_eq!(
                parse_command(line),
                Some(CropCommand::Usage(FD_CROP_USAGE)),
                "{line}"
            );
        }
    }

    #[test]
    fn a_setage_age_outside_a_byte_is_a_usage_error() {
        assert_eq!(
            parse_command("fdcrop setage 1 2 3 256"),
            Some(CropCommand::Usage(FD_CROP_USAGE))
        );
        // 0..=255 parses; the crop's own top age is checked where the block is
        // known, so the parser stays a syntax check.
        assert_eq!(
            parse_command("fdcrop setage 1 2 3 255"),
            Some(CropCommand::SetAge(AgeWrite {
                pos: [1, 2, 3],
                age: 255,
            }))
        );
    }
}
