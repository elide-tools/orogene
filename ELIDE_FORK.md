# Elide fork of orogene

- **Upstream:** https://github.com/orogene/orogene
- **Upstream commit at fork:** 2dc8d9e9d32b9dcc8e8a33e8a729c2c08772c33f
- **Upstream commit date:** 2024-11-11 14:49:15 -0800
- **Forked on:** 2026-04-23
- **Fork purpose:** embedded as `npm` resolver in WHIPLASH (`crates/resolvers`).

## Known build issues

None found as of fork date.

## Fork branch note

The upstream `main` HEAD (`7793618494db676bcfd3364b85fb21452a6c0bdf`, 2026-02-25)
contains a single commit titled "close source the project" that deleted all Rust source
and `Cargo.toml`. The fork branch `elide/main` is therefore based on the last
open-source commit (`2dc8d9e`), which is the parent of that deletion commit.

Upstream `main` can still be fetched for reference; do not merge it.

## Elide-applied patches

In reverse chronological order:

- `93018b4` (2026-04-23) — `feat(embed): add load_with_args + current_command_from for in-process embedding`
  - Adds `Orogene::load_with_args(argv: Vec<OsString>)` and the private helper `current_command_from(argv)` so embedders can invoke orogene with a caller-supplied argv instead of `std::env::args_os()`.
  - `--help` / `--version` flow through `try_get_matches_from` and return `Ok(())` rather than calling `process::exit`, so the embedder stays in control of process lifecycle.
  - Bumps workspace `indicatif` from `0.17.3` to `0.18` and relaxes `tracing-indicatif` to `"0.3"` so the version resolved inside the WHIPLASH workspace aligns with `tracing-indicatif 0.3.x` (which requires `indicatif 0.18`).
  - Files: `Cargo.toml`, `src/lib.rs`.

## Sync procedure

```bash
git fetch upstream
# Do NOT merge upstream/main — it contains the close-source deletion.
# If upstream releases new open-source commits on another branch, cherry-pick them:
# git cherry-pick <sha>
```

Always re-run `cargo build -p orogene` standalone after a sync before publishing the fork.
