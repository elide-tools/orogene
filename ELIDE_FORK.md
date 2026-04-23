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

None yet. Patches will be listed here as they are applied, in reverse chronological order.

## Sync procedure

```bash
git fetch upstream
# Do NOT merge upstream/main — it contains the close-source deletion.
# If upstream releases new open-source commits on another branch, cherry-pick them:
# git cherry-pick <sha>
```

Always re-run `cargo build -p orogene` standalone after a sync before publishing the fork.
