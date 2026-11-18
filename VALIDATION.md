# Validation

> **Historical (0.1.0, retired `.hnmod`).** This file records the ladder of the old container at
> Horizon `c9d474d`/`41f822b`/`1d63e0a5`. The current package (`api = "0.2"`, version `0.2.0`,
> 2026-11-18) is recorded in [`notes.md`](notes.md) → "Migration to the SDK format" and in
> `../_shared/evidence/farmersdelight/sdk-*.log`.

Validated on Linux with Rust 1.95.0, against the exact revisions in
`compatibility.json`: Horizon **c9d474d**
(`nightly-source/2026-11-01-c9d474d`) and horizon-mixins **d9c83d7**. Every
cargo/stage/e2e command ran with `CARGO_INCREMENTAL=0` and
`CARGO_BUILD_JOBS=1` or `2`.

## Standalone crate

| Check | Command | Result |
|---|---|---|
| fmt | `cargo fmt --all -- --check` | exit 0 |
| clippy | `cargo clippy --all-targets --locked --offline -- -D warnings` | exit 0, **0 warnings** |
| tests | `cargo test --locked --offline` | exit 0, **13 passed**, 0 failed |

## Staged pin

| Step | Result |
|---|---|
| `stage` | exit 0, revision `c9d474dd…`, **1 module, 0 injections** (`target/mixins/plan.json`); 235 s on the re-stage |
| `check` (direct cargo) | exit 0, **0 warnings**, 1 m 07 s (`target/direct-check.log`) |
| `test` (direct cargo) | exit 0, **13 passed / 0 failed / 531 filtered**, 24 m 17 s (`target/direct-test.log`); the built test binary re-ran 13/13 in 0.14 s |
| `horizon.py check` / `test` wrappers | started, but the machine was saturated by other ports' builds (load 34–47, 14 GiB nearly full); the wrapper's cargo command was still compiling at the 240–900 s timeouts, so no `cargo_exit` was recorded. The direct commands are the same cargo invocations; `tools/horizon.py` was updated to filter `game::farmersdelight::tests` (the template ships `game::example_mod::tests`) |
| `build` / `ctl` | **not run**: the port's `target/horizon-cargo/fast` was never built; a fast build under that load risked the OOM/reboot the campaign notes warn about. The e2e used the same-pin `measurements` fast client/ctl; this bundle has **no injections**, so the game binary is equivalent for content verification |

Environment finding: another job ran the campaign-wide
`ports/tools/clean-targets.sh` while this port's stage was live; it removed
`target/mixins` and SIGTERMed a build. The stage was recreated and the direct
test re-run. The new AGENTS rule says to clean only the finished port's target
while other jobs are running; this port's own target was cleaned with that
rule at the end of the job.

## Content `.hmod`

| Check | Result |
|---|---|
| `cargo-horizon-mod build` (from `content/`) | built `target/horizon-mod/farmersdelight.hmod` |
| `cargo-horizon-mod check content/farmersdelight.hmod --json` | `{"ok":true,…}`: id `farmersdelight`, version `1.21.1-1.3.4`, API 0.2, both, registries, unsigned, **382 files** |
| delivered file | [`content/farmersdelight.hmod`](content/farmersdelight.hmod), **134,336 bytes**, SHA-256 `7f1fa2bbacd05dbc6eee1c0cc03304910b2885b88f4b9e7574f0803d41d78a47` |

## In-game e2e (`tools/e2e.sh`)

| Run | Result |
|---|---|
| default | **exit 0, `PASS`**: 9/9 representative foods give, the tab lists 77/77, both recipes craft, `bread + stick` is empty, food 0 → 4 and saturation 0 → 3.2 on a fried egg, a stick changes neither, an unknown id is refused (`/tmp/opencode/fd-e2e-pre2.log`) |
| negative (`NEGATIVE=1`) | **exit 1, as required**: `FAIL: NEGATIVE run did not observe the false assertion (fried egg gave 4 food, expected the wrong 5)` (`/tmp/opencode/fd-e2e-negative.log`) |

The runs used `CTL`/`GAME` pointing at the `measurements` fast binaries (same
pin `c9d474d`) because this port's fast build was blocked by the load; the
content mod is this port's `content/`. Harness finding: `--creative` has no 2x2
grid, so the e2e switches to survival before the craft and eat phases.

## Embedded content + re-verification (2026-11-10)

Horizon `1d63e0a5a37a7a708eb0b32b49bf0dfeb28ed8a9` (game.rs `64813cf4…`),
horizon-mixins `5a64af938ec86b00d53d16a408aabc0038fb3e57` (schema 6), every step
through `ports/tools/build-slot.sh` with `CARGO_INCREMENTAL=0`.

| Check | Result |
|---|---|
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --all-targets --locked --offline -- -D warnings` | exit 0, 0 warnings |
| `cargo test --locked --offline` | green |
| `horizon.py stage` / `check` / `test` | exit 0 / `cargo_exit: 0`, 0 warnings / embedded tests green |
| `tools/package.sh` | `content/farmersdelight.hmod` 147,137 B `081378989c1b8f56…`, `farmersdelight.hnmod` 81,502 B `1f526bc750485a9c…` (8 entries, embeds the content) |
| e2e (no `--dev-mod`) | default `FAIL` + `EXIT:1` at “the cabbage never grew”; `NEGATIVE=1` `FAIL` + `EXIT:1` |
| `upstream_ready.py farmersdelight` | **BLOCKED — e2e has no PASS log** |

## Crop growth unblocked (2026-11-11/12)

Horizon `1d63e0a5a37a7a708eb0b32b49bf0dfeb28ed8a9`, horizon-mixins
`5a64af938ec86b00d53d16a408aabc0038fb3e57`; every build/test/e2e step through
`ports/tools/build-slot.sh` with `CARGO_INCREMENTAL=0`.

| Check | Result |
|---|---|
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --all-targets --locked --offline -- -D warnings` | exit 0, **0 warnings** |
| `cargo test --locked --offline` | exit 0, **20 passed**, 0 failed |
| `horizon.py check` | `cargo_exit: 0`, **0 warnings**, 2 m 08 s (`target/ladder/check.log`) |
| `horizon.py test` | `cargo_exit: 0`, **20 passed / 0 failed / 1240 filtered** (`target/ladder/test.log`) |
| `horizon.py build` | `cargo_exit: 0`, **0 warnings**, `fast` 12 m 44 s (`target/ladder/build.log`) |
| `tools/package.sh` (twice) | `content/farmersdelight.hmod` **147,172 B** `6222ca9df98bedd56ba229f211efa773405ab1b0a76eea9a2333e8ab416121e7` (418 entries); `farmersdelight.hnmod` **84,439 B** `cd453acd6cd389a2a2ae84b359b84d8acb1751d934c44cb8098f905adf03d982` (10 entries); identical on the second run |
| e2e default (`--dev-mod content/`) | **PASS + EXIT:0** (`ports/_shared/evidence/farmersdelight/e2e.log`) |
| e2e `NEGATIVE=1` | **FAIL + EXIT:1**, as required (`e2e-selftest.log`) |
| e2e `EMBEDDED=1` (no `--dev-mod`), `CONTROL=1` | **PASS + EXIT:0**; the unusable-`.horizon-cache` control reports `Unknown item 'farmersdelight:cabbage'` and `Unknown block type 'farmersdelight:cabbages'` (`e2e-embedded.log`) |
| `check-e2e-logs.py farmersdelight --strict` | `logs=5 ok=5 missing-exit=0 bad-exit=0`, exit 0 |
| `check-skills.py --notes ports/farmersdelight` | `SKILLS GATE: PASS` |
| `upstream_ready.py farmersdelight` | exit 0; still prints `blocked(crop-growth)` from the ledger row (the orchestrator updates it) |

The growth phase is fail-capable in both directions: `random_tick_speed 0` must
leave the crop at age 0 (it does), `random_tick_speed 1000` on hydrated farmland
must grow it (it does, to age 1 or 2 by the first five-tick sample), and each
gamerule set is read back with `set_rule` so a refused name cannot make the
phase vacuous (that refusal is what made the 2026-11-10 runs red).

## Registry packaging: version `0.1.0` (2026-11-12)

Horizon `1d63e0a5a37a7a708eb0b32b49bf0dfeb28ed8a9`, horizon-mixins
`5a64af938ec86b00d53d16a408aabc0038fb3e57`; every build/e2e step through
`ports/tools/build-slot.sh` with `CARGO_INCREMENTAL=0`.

The registry version is the port's own (`0.1.0`), so the entry, the artifact and
the release agree. The bump changes both artifact bytes, so the packaging and all
three e2e legs were re-run from the new sources.

| Check | Result |
|---|---|
| `tools/package.sh` (twice) | `content/farmersdelight.hmod` **147,172 B** `776569326f3e5be24ac5c60d69f964e1c6a8c1332b0bdaf20567312fd8bf0eb4` (418 entries); `farmersdelight.hnmod` **84,428 B** `84dbf6c52cba00240f094efdff3aa90df7d86a6d94ce55845f29959fd4abc4a1` (10 entries); `cmp` byte-identical between the runs |
| `cargo horizon-mod check farmersdelight.hnmod --kind native-mod` | exit 0, `farmersdelight 0.1.0 (native-mod, … for 26.3)`, 84,428 bytes, `84dbf6c5…`, unsigned |
| `cargo-horizon-mod check content/farmersdelight.hmod` | exit 0, `farmersdelight 0.1.0 (mod, API 0.2, both)`, `no code: content only` |
| e2e default (`--dev-mod content/`) | **PASS + EXIT:0** (`ports/_shared/evidence/farmersdelight/e2e.log`) |
| e2e `NEGATIVE=1` | **FAIL + EXIT:1**, as required (`e2e-selftest.log`) |
| e2e `EMBEDDED=1` (no `--dev-mod`), `CONTROL=1` | **PASS + EXIT:0**; the control reports the ids unknown (`e2e-embedded.log`) |
| `check-e2e-logs.py farmersdelight --strict` | `logs=5 ok=5 missing-exit=0 bad-exit=0`, exit 0 |
| `check-skills.py --notes ports/farmersdelight` | `SKILLS GATE: PASS` |
| `upstream_ready.py farmersdelight` | **READY**, exit 0 |
| `fix_icons.py` | `every publish tree has a compliant icon.png`, exit 0 |
