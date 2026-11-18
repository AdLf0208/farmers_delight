#!/usr/bin/env python3
"""Check that the Farmer's Delight creative tab lists the slice foods.

Reads the game's merged content cache
(`<game>/.horizon-cache/mod-content/creative-inventory.json`), prints the tab
ids and the mod's own tab, and fails with the missing ids. Called by
tools/e2e.sh.
"""
import json
import sys


def main() -> int:
    path, *wanted = sys.argv[1:]
    with open(path) as handle:
        inventory = json.load(handle)
    tabs = inventory.get("tabs", [])
    print("tabs:", [tab.get("id") for tab in tabs])
    ours = [tab for tab in tabs if str(tab.get("id", "")).startswith("farmersdelight:")]
    for tab in ours:
        print("farmersdelight tab:", json.dumps(tab, sort_keys=True))
    if not ours:
        print(f"FAIL: no farmersdelight tab in {path}", file=sys.stderr)
        return 1
    listed = [item for tab in ours for item in tab.get("items") or []]
    missing = [item for item in wanted if not any(f'id:"{item}"' in entry for entry in listed)]
    for item in missing:
        print(f"FAIL: the farmersdelight tab misses {item}", file=sys.stderr)
    if not missing:
        print(f"PASS: the farmersdelight tab lists all {len(wanted)} items")
    return 1 if missing else 0


if __name__ == "__main__":
    raise SystemExit(main())
