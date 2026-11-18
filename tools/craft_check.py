#!/usr/bin/env python3
"""Craft Farmer's Delight slice-1 recipes in the inventory grid.

Called by tools/e2e.sh with the open game's control socket. Uses the
harness's 2x2 inventory grid (`ctl screen`: slots 100-103, result 200).

Cases:
1. bread + two fried eggs -> one `farmersdelight:egg_sandwich` (the
   `c:foods/bread` + `c:foods/cooked_egg` tags);
2. three wheat + one egg -> three `farmersdelight:wheat_dough` (the
   `c:crops/wheat` + `c:eggs` tags and the count-3 result);
3. bread + a stick -> an empty result (the control: the port's ingredients
   are what make the other cases match).

Exits 1 with the observed result on the first mismatch.
"""
import json
import subprocess
import sys


def main() -> int:
    sock, ctl = sys.argv[1], sys.argv[2]

    def run(*args):
        out = subprocess.run([ctl, "--socket", sock, *args], text=True, capture_output=True)
        try:
            return json.loads(out.stdout)
        except json.JSONDecodeError:
            raise SystemExit(f"ctl {' '.join(args)}: {out.stdout}{out.stderr}")

    def wait(ticks=4):
        run("wait", str(ticks))

    def slots():
        screen = run("screen")
        if "slots" not in screen:
            raise SystemExit(f"no menu is open: {screen}")
        return screen["slots"]

    def find(item):
        for slot in slots():
            if slot.get("item") == item:
                return slot["slot"]
        raise SystemExit(f"missing ingredient {item} in the inventory")

    def result():
        stack = [s for s in slots() if s["slot"] == 200]
        if not stack or not stack[0].get("item"):
            return None
        return stack[0]

    def reset(items):
        try:
            run("close")
        except SystemExit:
            pass
        wait(3)
        run("command", "clear @s")
        for item, count in items:
            run("give", item, str(count))
        wait(3)
        run("open-inventory")
        wait(4)

    def place(item, mode, targets):
        slot = find(item)
        run("click-slot", str(slot))
        wait(1)
        for target in targets:
            if mode == "right":
                run("click-slot", str(target), "right")
            else:
                run("click-slot", str(target))
            wait(1)
        run("click-slot", str(slot))
        wait(2)

    def check(name, items, placements, expected_item, expected_count):
        reset(items)
        for item, mode, targets in placements:
            place(item, mode, targets)
        wait(3)
        got = result()
        print(name, "->", json.dumps(got))
        if got is None:
            raise SystemExit(f"FAIL: {name} produced no result")
        if got.get("item") != expected_item or got.get("count") != expected_count:
            raise SystemExit(
                f"FAIL: {name} produced {got.get('item')} x{got.get('count')}, "
                f"expected {expected_item} x{expected_count}"
            )
        run("close")
        wait(2)

    def check_empty(name, items, placements):
        reset(items)
        for item, mode, targets in placements:
            place(item, mode, targets)
        wait(3)
        got = result()
        print(name, "->", json.dumps(got))
        if got is not None:
            raise SystemExit(f"FAIL: {name} produced {got.get('item')}")
        run("close")
        wait(2)

    check(
        "bread + two fried eggs",
        [("minecraft:bread", 1), ("farmersdelight:fried_egg", 2)],
        [
            ("minecraft:bread", "left", ["100"]),
            ("farmersdelight:fried_egg", "right", ["101", "102"]),
        ],
        "farmersdelight:egg_sandwich",
        1,
    )
    check(
        "three wheat + one egg",
        [("minecraft:wheat", 3), ("minecraft:egg", 1)],
        [
            ("minecraft:wheat", "right", ["100", "101", "102"]),
            ("minecraft:egg", "left", ["103"]),
        ],
        "farmersdelight:wheat_dough",
        3,
    )
    check_empty(
        "bread + stick",
        [("minecraft:bread", 1), ("minecraft:stick", 1)],
        [
            ("minecraft:bread", "left", ["100"]),
            ("minecraft:stick", "left", ["101"]),
        ],
    )
    print("PASS: both recipes craft and the control pair is empty")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
