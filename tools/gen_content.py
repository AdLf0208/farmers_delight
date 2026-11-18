#!/usr/bin/env python3
"""Generate the Farmer's Delight carried content for the SDK package.

The canonical table is the 77 edible items of Farmer's Delight 1.21.1-1.3.4,
parsed from `common/FoodValues.java` and `common/registry/ModItems.java` at
commit dd8b9b0. 1.21.1's `saturationModifier(m)` is converted to 26.3's
absolute `saturation` as `nutrition * m * 2` (the builder's
`FoodConstants.saturationByModifier`); `KELP_ROLL` uses the direct constructor
and keeps its absolute 12.

Run against the analysis clone:

    python3 tools/gen_content.py --upstream ../_shared/upstream/farmersdelight

It writes `assets/farmersdelight/` and `data/{farmersdelight,c}/` at the package
root (the port directory), beside `src/` and the hand-written `horizon-mod.toml`;
only those two carried trees are replaced. The generated assets are the upstream
MIT textures/models/lang; the port is local-only until publication is approved.
"""
import argparse
import json
import shutil
from pathlib import Path

FOODS = [
    {"name": "cabbage", "nutrition": 2, "saturation": 1.60, "stack": 64, "consume_seconds": 1.6, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "tomato", "nutrition": 1, "saturation": 0.60, "stack": 64, "consume_seconds": 1.6, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "onion", "nutrition": 2, "saturation": 1.60, "stack": 64, "consume_seconds": 1.6, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "fried_egg", "nutrition": 4, "saturation": 3.20, "stack": 64, "consume_seconds": 1.6, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "apple_cider", "nutrition": 0, "saturation": 0.00, "stack": 16, "consume_seconds": 1.6, "always_edible": True, "remainder": 'minecraft:glass_bottle', "drink": True, "effects": [('minecraft:absorption', 1200, 0, 1.0)]},
    {"name": "tomato_sauce", "nutrition": 4, "saturation": 3.20, "stack": 64, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "wheat_dough", "nutrition": 2, "saturation": 1.20, "stack": 64, "consume_seconds": 1.6, "always_edible": False, "remainder": None, "drink": False, "effects": [('minecraft:hunger', 600, 0, 0.3)]},
    {"name": "raw_pasta", "nutrition": 2, "saturation": 1.20, "stack": 64, "consume_seconds": 1.6, "always_edible": False, "remainder": None, "drink": False, "effects": [('minecraft:hunger', 600, 0, 0.3)]},
    {"name": "pumpkin_slice", "nutrition": 3, "saturation": 1.80, "stack": 64, "consume_seconds": 1.6, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "cabbage_leaf", "nutrition": 1, "saturation": 0.80, "stack": 64, "consume_seconds": 0.8, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "minced_beef", "nutrition": 2, "saturation": 1.20, "stack": 64, "consume_seconds": 0.8, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "beef_patty", "nutrition": 4, "saturation": 6.40, "stack": 64, "consume_seconds": 0.8, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "chicken_cuts", "nutrition": 1, "saturation": 0.60, "stack": 64, "consume_seconds": 0.8, "always_edible": False, "remainder": None, "drink": False, "effects": [('minecraft:hunger', 600, 0, 0.3)]},
    {"name": "cooked_chicken_cuts", "nutrition": 3, "saturation": 3.60, "stack": 64, "consume_seconds": 0.8, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "bacon", "nutrition": 2, "saturation": 1.20, "stack": 64, "consume_seconds": 0.8, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "cooked_bacon", "nutrition": 4, "saturation": 6.40, "stack": 64, "consume_seconds": 0.8, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "cod_slice", "nutrition": 1, "saturation": 0.20, "stack": 64, "consume_seconds": 0.8, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "cooked_cod_slice", "nutrition": 3, "saturation": 3.00, "stack": 64, "consume_seconds": 0.8, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "salmon_slice", "nutrition": 1, "saturation": 0.20, "stack": 64, "consume_seconds": 0.8, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "cooked_salmon_slice", "nutrition": 3, "saturation": 4.80, "stack": 64, "consume_seconds": 0.8, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "mutton_chops", "nutrition": 1, "saturation": 0.60, "stack": 64, "consume_seconds": 0.8, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "cooked_mutton_chops", "nutrition": 3, "saturation": 4.80, "stack": 64, "consume_seconds": 0.8, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "ham", "nutrition": 5, "saturation": 3.00, "stack": 64, "consume_seconds": 1.6, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "smoked_ham", "nutrition": 10, "saturation": 16.00, "stack": 64, "consume_seconds": 1.6, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "pie_crust", "nutrition": 2, "saturation": 0.80, "stack": 64, "consume_seconds": 1.6, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "cake_slice", "nutrition": 2, "saturation": 0.40, "stack": 64, "consume_seconds": 0.8, "always_edible": False, "remainder": None, "drink": False, "effects": [('minecraft:speed', 400, 0, 1.0)]},
    {"name": "apple_pie_slice", "nutrition": 3, "saturation": 1.80, "stack": 64, "consume_seconds": 0.8, "always_edible": False, "remainder": None, "drink": False, "effects": [('minecraft:speed', 600, 0, 1.0)]},
    {"name": "sweet_berry_cheesecake_slice", "nutrition": 3, "saturation": 1.80, "stack": 64, "consume_seconds": 0.8, "always_edible": False, "remainder": None, "drink": False, "effects": [('minecraft:speed', 600, 0, 1.0)]},
    {"name": "chocolate_pie_slice", "nutrition": 3, "saturation": 1.80, "stack": 64, "consume_seconds": 0.8, "always_edible": False, "remainder": None, "drink": False, "effects": [('minecraft:speed', 600, 0, 1.0)]},
    {"name": "pumpkin_pie_slice", "nutrition": 3, "saturation": 1.80, "stack": 64, "consume_seconds": 0.8, "always_edible": False, "remainder": None, "drink": False, "effects": [('minecraft:speed', 600, 0, 1.0)]},
    {"name": "sweet_berry_cookie", "nutrition": 2, "saturation": 0.40, "stack": 64, "consume_seconds": 0.8, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "honey_cookie", "nutrition": 2, "saturation": 0.40, "stack": 64, "consume_seconds": 0.8, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "melon_popsicle", "nutrition": 3, "saturation": 1.20, "stack": 64, "consume_seconds": 0.8, "always_edible": True, "remainder": None, "drink": False, "effects": []},
    {"name": "glow_berry_custard", "nutrition": 7, "saturation": 8.40, "stack": 16, "consume_seconds": 1.6, "always_edible": True, "remainder": 'minecraft:glass_bottle', "drink": False, "effects": [('minecraft:glowing', 100, 0, 1.0)]},
    {"name": "fruit_salad", "nutrition": 6, "saturation": 7.20, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": [('minecraft:regeneration', 100, 0, 1.0)]},
    {"name": "mixed_salad", "nutrition": 6, "saturation": 7.20, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": [('minecraft:regeneration', 100, 0, 1.0)]},
    {"name": "nether_salad", "nutrition": 5, "saturation": 4.00, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": [('minecraft:nausea', 240, 0, 0.3)]},
    {"name": "barbecue_stick", "nutrition": 8, "saturation": 14.40, "stack": 64, "consume_seconds": 1.6, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "egg_sandwich", "nutrition": 8, "saturation": 12.80, "stack": 64, "consume_seconds": 1.6, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "chicken_sandwich", "nutrition": 10, "saturation": 16.00, "stack": 64, "consume_seconds": 1.6, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "hamburger", "nutrition": 11, "saturation": 17.60, "stack": 64, "consume_seconds": 1.6, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "bacon_sandwich", "nutrition": 10, "saturation": 16.00, "stack": 64, "consume_seconds": 1.6, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "mutton_wrap", "nutrition": 10, "saturation": 16.00, "stack": 64, "consume_seconds": 1.6, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "dumplings", "nutrition": 8, "saturation": 12.80, "stack": 64, "consume_seconds": 1.6, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "stuffed_potato", "nutrition": 10, "saturation": 14.00, "stack": 64, "consume_seconds": 1.6, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "cabbage_rolls", "nutrition": 5, "saturation": 5.00, "stack": 64, "consume_seconds": 1.6, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "salmon_roll", "nutrition": 7, "saturation": 8.40, "stack": 64, "consume_seconds": 1.6, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "cod_roll", "nutrition": 7, "saturation": 8.40, "stack": 64, "consume_seconds": 1.6, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "kelp_roll", "nutrition": 12, "saturation": 12.00, "stack": 64, "consume_seconds": 2.4, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "kelp_roll_slice", "nutrition": 6, "saturation": 6.00, "stack": 64, "consume_seconds": 0.8, "always_edible": False, "remainder": None, "drink": False, "effects": []},
    {"name": "cooked_rice", "nutrition": 6, "saturation": 4.80, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "bone_broth", "nutrition": 8, "saturation": 11.20, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": True, "effects": []},
    {"name": "beef_stew", "nutrition": 12, "saturation": 19.20, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "chicken_soup", "nutrition": 12, "saturation": 19.20, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "vegetable_soup", "nutrition": 12, "saturation": 19.20, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "fish_stew", "nutrition": 12, "saturation": 19.20, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "fried_rice", "nutrition": 12, "saturation": 19.20, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "pumpkin_soup", "nutrition": 14, "saturation": 21.00, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "baked_cod_stew", "nutrition": 14, "saturation": 21.00, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "noodle_soup", "nutrition": 14, "saturation": 21.00, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "onion_soup", "nutrition": 12, "saturation": 19.20, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "bacon_and_eggs", "nutrition": 10, "saturation": 12.00, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "pasta_with_meatballs", "nutrition": 12, "saturation": 19.20, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "pasta_with_mutton_chop", "nutrition": 12, "saturation": 19.20, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "mushroom_rice", "nutrition": 12, "saturation": 19.20, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "roasted_mutton_chops", "nutrition": 14, "saturation": 21.00, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "vegetable_noodles", "nutrition": 14, "saturation": 21.00, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "steak_and_potatoes", "nutrition": 12, "saturation": 19.20, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "ratatouille", "nutrition": 10, "saturation": 12.00, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "squid_ink_pasta", "nutrition": 14, "saturation": 21.00, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "grilled_salmon", "nutrition": 14, "saturation": 21.00, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "roast_chicken", "nutrition": 14, "saturation": 21.00, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "stuffed_pumpkin", "nutrition": 14, "saturation": 21.00, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "honey_glazed_ham", "nutrition": 14, "saturation": 21.00, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "shepherds_pie", "nutrition": 14, "saturation": 21.00, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "gleaming_salad", "nutrition": 14, "saturation": 21.00, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
    {"name": "dog_food", "nutrition": 4, "saturation": 1.60, "stack": 16, "consume_seconds": 1.6, "always_edible": False, "remainder": 'minecraft:bowl', "drink": False, "effects": []},
]


# 1.21.1 recipes the slice carries: vanilla crafting and vanilla cooking
# stations. The mod's own cooking-pot and cutting-board recipe types are
# deferred with those blocks.
RECIPE_TYPES = {
    "minecraft:crafting_shapeless",
    "minecraft:crafting_shaped",
    "minecraft:smelting",
    "minecraft:smoking",
    "minecraft:campfire_cooking",
}

# The `c:` item tags the carried recipes name. 26.3 ships no `c:` tags, so the
# port supplies the exact members that exist in this slice plus vanilla.
C_TAGS = {
    "crops/beetroot": ["minecraft:beetroot"],
    "crops/cabbage": ["farmersdelight:cabbage"],
    "crops/carrot": ["minecraft:carrot"],
    "crops/onion": ["farmersdelight:onion"],
    "crops/potato": ["minecraft:potato"],
    "crops/tomato": ["farmersdelight:tomato"],
    "crops/wheat": ["minecraft:wheat"],
    "drinks/milk": ["minecraft:milk_bucket"],
    "eggs": ["minecraft:egg"],
    "foods/berry": ["minecraft:sweet_berries", "minecraft:glow_berries"],
    "foods/bread": ["minecraft:bread"],
    "foods/cooked_bacon": ["farmersdelight:cooked_bacon"],
    "foods/cooked_beef": ["minecraft:cooked_beef"],
    "foods/cooked_chicken": ["minecraft:cooked_chicken"],
    "foods/cooked_egg": ["farmersdelight:fried_egg"],
    "foods/cooked_meat": [
        "minecraft:cooked_beef",
        "minecraft:cooked_porkchop",
        "minecraft:cooked_mutton",
        "minecraft:cooked_chicken",
        "minecraft:cooked_rabbit",
        "farmersdelight:cooked_bacon",
    ],
    "foods/cooked_mutton": ["minecraft:cooked_mutton"],
    "foods/cooked_salmon": ["minecraft:cooked_salmon"],
    "foods/leafy_green": ["farmersdelight:cabbage_leaf"],
    "foods/pasta": ["farmersdelight:raw_pasta"],
    "foods/vegetable": [
        "minecraft:carrot",
        "minecraft:potato",
        "minecraft:beetroot",
        "farmersdelight:cabbage",
        "farmersdelight:tomato",
        "farmersdelight:onion",
        "farmersdelight:cabbage_leaf",
        "farmersdelight:pumpkin_slice",
    ],
}

# Upstream item tags that still make sense without the deferred blocks.
ITEM_TAGS = ["meals", "snacks", "sweets", "drinks"]


def food_component(food):
    component = {
        "nutrition": food["nutrition"],
        "saturation": food["saturation"],
    }
    if food["always_edible"]:
        component["can_always_eat"] = True
    return component


def consumable(food):
    effects = [
        {
            "type": "minecraft:apply_effects",
            "effects": [
                {"id": effect_id, "duration": duration, "amplifier": amplifier}
            ],
            "probability": probability,
        }
        for effect_id, duration, amplifier, probability in food["effects"]
    ]
    if (
        not effects
        and food["consume_seconds"] == 1.6
        and not food["drink"]
    ):
        return None
    value = {}
    if food["drink"]:
        # `DrinkableItem` changes the animation and keeps vanilla's consume
        # particles; only the sound follows the drink animation.
        value.update(
            {
                "animation": "drink",
                "sound": "minecraft:entity.generic.drink",
            }
        )
    if food["consume_seconds"] != 1.6:
        value["consume_seconds"] = food["consume_seconds"]
    if effects:
        value["on_consume_effects"] = effects
    return value


def item_json(food):
    components = {
        # `minecraft:apple` is compostable at 30 %; the slice carries no
        # compost values, so it must not inherit the template's.
        "!minecraft:compostable": {},
        "minecraft:food": food_component(food),
    }
    use = consumable(food)
    if use is not None:
        components["minecraft:consumable"] = use
    if food["remainder"]:
        components["minecraft:use_remainder"] = {"id": food["remainder"]}
    item = {"template": "minecraft:apple", "components": components}
    if food["stack"] != 64:
        item["max_stack_size"] = food["stack"]
    return item


def model_parent(name):
    if name == "apple_cider":
        return "farmersdelight:item/mug"
    if name in {"barbecue_stick", "ham", "smoked_ham"}:
        return "minecraft:item/handheld"
    return "minecraft:item/generated"


def ingredient(value):
    if isinstance(value, str):
        return value
    if "item" in value:
        return value["item"]
    if "tag" in value:
        return "#" + value["tag"]
    raise ValueError(f"unsupported ingredient {value!r}")


def convert_recipe(recipe):
    kind = recipe["type"]
    if kind not in RECIPE_TYPES:
        return None
    out = {"type": kind}
    for key in ("category", "group", "cookingtime", "experience"):
        if key in recipe:
            out[key] = recipe[key]
    if "ingredients" in recipe:
        out["ingredients"] = [ingredient(value) for value in recipe["ingredients"]]
    if "key" in recipe:
        out["key"] = {char: ingredient(value) for char, value in recipe["key"].items()}
    if "pattern" in recipe:
        out["pattern"] = recipe["pattern"]
    if "ingredient" in recipe:
        out["ingredient"] = ingredient(recipe["ingredient"])
    result = recipe["result"]
    out["result"] = {"id": result["id"], "count": result.get("count", 1)}
    return out


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--upstream", type=Path, required=True)
    parser.add_argument("--out", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    upstream = args.upstream.resolve()
    out = args.out.resolve()
    foods = {food["name"]: food for food in FOODS}
    if len(foods) != len(FOODS):
        raise SystemExit("duplicate food name")

    # The package root is the port itself (the SDK layout): `assets/` and `data/`
    # sit beside `src/`, and the manifest there is hand-written, so only the carried
    # content trees are regenerated — never the root.
    for carried in ("assets", "data"):
        if (out / carried).exists():
            shutil.rmtree(out / carried)
    (out / "data/farmersdelight/horizon/item").mkdir(parents=True)
    (out / "data/farmersdelight/horizon/creative_tab").mkdir(parents=True)
    (out / "data/farmersdelight/recipe").mkdir(parents=True)
    (out / "assets/farmersdelight/items").mkdir(parents=True)
    (out / "assets/farmersdelight/models/item").mkdir(parents=True)
    (out / "assets/farmersdelight/textures/item").mkdir(parents=True)
    (out / "assets/farmersdelight/lang").mkdir(parents=True)

    for name, food in foods.items():
        (out / f"data/farmersdelight/horizon/item/{name}.json").write_text(
            json.dumps(item_json(food), indent=2, sort_keys=True) + "\n"
        )
        (out / f"assets/farmersdelight/items/{name}.json").write_text(
            json.dumps(
                {"model": {"type": "minecraft:model", "model": f"farmersdelight:item/{name}"}},
                indent=2,
                sort_keys=True,
            )
            + "\n"
        )
        (out / f"assets/farmersdelight/models/item/{name}.json").write_text(
            json.dumps(
                {
                    "parent": model_parent(name),
                    "textures": {"layer0": f"farmersdelight:item/{name}"},
                },
                indent=2,
                sort_keys=True,
            )
            + "\n"
        )
        texture = upstream / f"src/main/resources/assets/farmersdelight/textures/item/{name}.png"
        if not texture.is_file():
            raise SystemExit(f"missing upstream texture {texture}")
        shutil.copyfile(texture, out / f"assets/farmersdelight/textures/item/{name}.png")

    mug = upstream / "src/main/resources/assets/farmersdelight/models/item/mug.json"
    shutil.copyfile(mug, out / "assets/farmersdelight/models/item/mug.json")

    lang = json.loads(
        (upstream / "src/main/resources/assets/farmersdelight/lang/en_us.json").read_text()
    )
    keys = {f"item.farmersdelight.{name}": lang[f"item.farmersdelight.{name}"] for name in foods}
    keys["itemGroup.farmersdelight"] = lang["itemGroup.farmersdelight"]
    (out / "assets/farmersdelight/lang/en_us.json").write_text(
        json.dumps(keys, indent=2, sort_keys=True) + "\n"
    )

    tab = {
        "title": "itemGroup.farmersdelight",
        "icon": "farmersdelight:hamburger",
        "items": [f"farmersdelight:{name}" for name in foods],
    }
    (out / "data/farmersdelight/horizon/creative_tab/farmersdelight.json").write_text(
        json.dumps(tab, indent=2, sort_keys=True) + "\n"
    )

    needed_tags = set()
    kept = 0
    recipes = upstream / "src/generated/resources/data/farmersdelight/recipe"
    for path in sorted(recipes.rglob("*.json")):
        recipe = json.loads(path.read_text())
        if recipe.get("type") not in RECIPE_TYPES:
            continue
        result = recipe.get("result", {})
        if not isinstance(result, dict):
            continue
        result_id = result.get("id", "")
        if not result_id.startswith("farmersdelight:"):
            continue
        if result_id.split(":", 1)[1] not in foods:
            continue
        try:
            converted = convert_recipe(recipe)
        except ValueError:
            continue
        if converted is None:
            continue
        # A crafting recipe whose ingredient is one of the deferred non-food
        # items (a crate) cannot resolve, so it is not carried.
        ingredients = []
        if "ingredients" in converted:
            ingredients = converted["ingredients"]
        if "ingredient" in converted:
            ingredients = [converted["ingredient"]]
        if "key" in converted:
            ingredients = list(converted["key"].values())
        deferred = [
            value.lstrip("#")
            for value in ingredients
            if value.lstrip("#").startswith("farmersdelight:")
            and value.lstrip("#").split(":", 1)[1] not in foods
        ]
        if deferred:
            continue
        for value in ingredients:
            if value.startswith("#"):
                needed_tags.add(value[1:])
        (out / f"data/farmersdelight/recipe/{path.stem}.json").write_text(
            json.dumps(converted, indent=2, sort_keys=True) + "\n"
        )
        kept += 1

    missing = sorted(tag for tag in needed_tags if not tag.startswith("c:") or tag[2:] not in C_TAGS)
    if missing:
        raise SystemExit(f"carried recipes need tags the generator does not supply: {missing}")
    for tag, values in C_TAGS.items():
        path = out / f"data/c/tags/item/{tag}.json"
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps({"values": values}, indent=2, sort_keys=True) + "\n")

    for tag in ITEM_TAGS:
        source = upstream / f"src/generated/resources/data/farmersdelight/tags/item/{tag}.json"
        values = json.loads(source.read_text())["values"]
        known = [value for value in values if value.startswith("minecraft:") or value.split(":", 1)[1] in foods]
        path = out / f"data/farmersdelight/tags/item/{tag}.json"
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps({"values": known}, indent=2, sort_keys=True) + "\n")

    print(f"wrote {len(foods)} foods, {kept} recipes, {len(C_TAGS)} c: tags")


if __name__ == "__main__":
    main()
