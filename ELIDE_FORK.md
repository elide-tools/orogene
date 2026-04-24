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

- (2026-04-23) — `feat: gate dialoguer behind interactive feature`
  - Root `Cargo.toml`: added `interactive` feature (default-on) that activates `dep:dialoguer` and `oro-npm-account/interactive`. Marked `dialoguer` optional.
  - `crates/oro-npm-account/Cargo.toml`: added matching `interactive` feature (default-on), `dialoguer` optional.
  - `src/lib.rs`: `prompt_telemetry_opt_in` now has two impls — real one gated by `#[cfg(feature = "interactive")]`, stub returning `Ok(false)` (auto-decline) when off. Dialoguer imports cfg-gated.
  - `crates/oro-npm-account/src/login.rs`: Legacy auth flow (username/password prompts) returns `OroNpmAccountError::InteractiveFeatureDisabled` when the feature is off; Web auth flow remains fully functional.
  - `crates/oro-npm-account/src/error.rs`: added `InteractiveFeatureDisabled` variant.
  - Drops `dialoguer` from the dep graph when `--no-default-features` is used (as WHIPLASH does).

- (2026-04-23) — `fix(nassun): target-gate tsify to wasm32 only`
  - `crates/nassun/Cargo.toml`: moved `tsify` from the unconditional `[dependencies]` section to the `[target.'cfg(target_arch = "wasm32")'.dependencies]` block alongside its `wasm-bindgen` family siblings. The only usage of `tsify::Tsify` is in `crates/nassun/src/wasm.rs`, which is already `#[cfg(target_arch = "wasm32")]`-gated from `lib.rs`. The dep was pulled into every target build purely as an oversight.
  - Drops `tsify`, `wasm-bindgen`, `serde-wasm-bindgen`, `js-sys`, `wasm-streams`, and `console_error_panic_hook` from the non-wasm build graph (was 5+ crates). No cargo feature needed — just correct target gating.

- (2026-04-23) — `feat: gate global tracing-subscriber install`
  - `Cargo.toml`: added feature `install-tracing-subscriber` to `[features]`, default-on. No new deps.
  - `src/lib.rs`: `Orogene::setup_logging` has two impls now — the real one gated by `#[cfg(feature = "install-tracing-subscriber")]`, and a stub that returns `Ok(None)` when the feature is off. The stub lets tracing events continue to emit through the macros without installing a global subscriber, so the embedder can own the subscriber and receive events alongside other emitters (uv, Elide, …).

- (2026-04-23) — `feat: gate sentry behind a cargo feature`
  - `Cargo.toml`: added `[features]` section with `default = ["sentry"]` and `sentry = ["dep:sentry"]`. Made the `sentry` dep `optional = true`.
  - `src/lib.rs`: `Orogene::setup_telemetry` is now feature-gated. A no-op stub (`#[cfg(not(feature = "sentry"))]`) returning `Ok(None)` replaces the real impl when the feature is off. The two remaining call sites (`sentry::configure_scope`/`sentry::capture_error` in the error-handling closure of `load`) are wrapped in `#[cfg(feature = "sentry")]`.
  - Consumers that embed orogene via `default-features = false` no longer pull `sentry` into the dep graph at all. Standalone `cargo build -p orogene` preserves prior behavior (sentry on).

- (2026-04-23) — `chore: drop sentry transport features (ureq/rustls)`
  - `Cargo.toml` workspace: `sentry` feature list reduced from `["backtrace", "contexts", "debug-images", "panic", "ureq", "rustls"]` to `["backtrace", "contexts", "debug-images", "panic"]`.
  - Reason: sentry 0.31's `ureq` transport requires `ureq::rustls` which hard-pins an older `rustls` version (0.21) that conflicts with the modern `rustls` (0.23) pulled by reqwest 0.12. Dropping the transport makes sentry a stub — panic handlers still install and crash data is still captured, but no events are emitted to a remote. This is acceptable for an embedded resolver whose crash reporting is the embedder's responsibility (Elide has its own telemetry layer).

- (2026-04-23) — `chore: bump reqwest 0.11 → 0.12 + middleware 0.4`
  - `Cargo.toml` workspace: `reqwest 0.11.14` → `reqwest 0.12` (default-features = false, features = rustls-tls/json/gzip/brotli/stream); `reqwest-middleware =0.2.2` → `0.4`; `reqwest-retry 0.2.2` → `0.7`; `http-cache-reqwest 0.6.0` → `0.15`; added `http = "1"` workspace dep; removed `task-local-extensions` workspace dep (no longer needed — reqwest-middleware 0.4 uses `http::Extensions` instead).
  - `Cargo.toml` workspace: `sentry 0.31.0` switched to `default-features = false, features = ["backtrace", "contexts", "debug-images", "panic", "ureq", "rustls"]`. Sentry 0.31's `transport` feature hard-wires `native-tls`; using `ureq` + `rustls` instead gives a rustls-only transport.
  - `Cargo.lock`: tokio bumped 1.32 → 1.52.1 (required by `tokio-stream 0.1.18` which was pulled transitively by `http-cache 0.20.1` → `cacache 13.1.0`); native-tls, hyper-tls, tokio-native-tls, task-local-extensions removed; reqwest 0.11 removed; reqwest 0.12.28 locked.
  - `crates/oro-client/Cargo.toml`: removed `task-local-extensions` dep; added `http` workspace dep; simplified `reqwest` dep (features now in workspace def); bumped other deps via workspace.
  - `crates/oro-client/src/auth_middleware.rs`: replaced `use task_local_extensions::Extensions` with `use http::Extensions` to match `reqwest-middleware 0.4` trait signature (the `Middleware::handle` signature is otherwise identical).
  - `crates/oro-client/src/client.rs`: updated `http_cache_reqwest` import to add `HttpCacheOptions`; `options: None` → `options: HttpCacheOptions::default()` (API change in http-cache-reqwest 0.15); `CACacheManager { path: String }` → `CACacheManager { path: PathBuf }` (path field type changed in http-cache 0.20).
  - **Result:** `cargo tree -p orogene | grep native-tls` returns nothing. Single reqwest version (0.12.28) in graph.

- (2026-04-23) — `chore: remove dead syn workspace pin`
  - `Cargo.toml` workspace: removed `syn = "1.0.33"`. No crate in the orogene tree directly depends on `syn` — the pin was unused. Transitive `syn 2.x` is now pulled by `thiserror-impl` and `serde_derive` unambiguously.

- (2026-04-23) — `chore: bump thiserror 1 → 2`
  - `Cargo.toml` workspace: `thiserror = "1.0.38"` → `thiserror = "2"` (resolves to 2.0.18).
  - `Cargo.lock`: transitively bumped `time 0.3.29` → `0.3.47` (rustc 1.94 type-inference regression in `time 0.3.29`) and `wasm-bindgen 0.2.87` → `0.2.118` (incompatible with rustc 1.94); also bumped `serde 1.0.188` → `1.0.228` and `proc-macro2 1.0.68` → `1.0.106` as indirect consequences.
  - `crates/nassun/src/error.rs`: two `#[error(...)]` format strings used a mixed positional-index + expression pattern (`{2}{}` with `if let Some(path) = .1`) that thiserror 2 rejects as ambiguous. Changed to `{_N}` / `_N` named binding style on `ExtractIoError` (line 53) and `ExtractCacheError` (line 64). No semantic change.

- (2026-04-23) — `chore: bump rust-toolchain 1.72 → 1.94; switch reqwest to rustls-tls`
  - `rust-toolchain.toml`: channel `1.72.1` → `1.94.0`. The 1.72 pin blocked edition-2024 deps (notably `tracing-indicatif 0.3.13` which aligns on `indicatif 0.18`). 1.94 matches uv's pin and WHIPLASH's MSRV.
  - `Cargo.toml` workspace: `reqwest = "0.11.14"` → `reqwest = { version = "0.11.14", default-features = false, features = ["rustls-tls"] }`. Disables the default `default-tls` feature (which pulls `native-tls`/`hyper-tls`) at the workspace level.
  - `Cargo.lock`: `tracing-indicatif` updated to `0.3.13`, transitively aligning `indicatif` to `0.18.4` (eliminating the prior 0.17/0.18 split in the standalone lock).
  - **Caveat:** `reqwest-middleware 0.2.2` (transitive via `http-cache-reqwest`) still enables reqwest's default features on its own dep line, so `native-tls` still appears in the resolved graph. True rustls-only convergence requires bumping `reqwest-middleware` to 0.4 and `reqwest` to 0.13. Tracked as follow-up.

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
