# Changelog

All notable changes to this crate are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1] - 2026-07-20

### Fixed

- Every derive macro emitted a hardcoded `ddd_toolkit_core::domain::...` path
  in its generated code, which only resolved if the invoking crate depended
  on `ddd-toolkit-core` directly. A downstream user depending only on the
  `ddd-toolkit` facade got `error[E0433]: cannot find module or crate
  ddd_toolkit_core` on the first use of any derive. `core_crate_path()` now
  resolves, per macro invocation, whether the calling crate depends on
  `ddd-toolkit-core` directly or only through the facade, and emits the
  matching path.
- The `ddd-toolkit-core` git dev-dependency now uses HTTPS instead of SSH, so
  CI (and anyone building this crate's own tests without an SSH key) can
  fetch it.

### Added

- Crates.io metadata (`license`, `description`, `repository`, `documentation`,
  `readme`, `keywords`, `categories`), `LICENSE-MIT`/`LICENSE-APACHE`, and a
  `README.md` documenting every derive and `#[vo(...)]` attribute, in
  preparation for the first crates.io release.
- Crate-level and per-derive docs closing every `missing_docs` gap on the
  public API surface.

### Changed

- Moved the `ddd-toolkit-core` dependency from `[dependencies]` to
  `[dev-dependencies]`: nothing in `src/` needs it compiled in (the derives
  only reference it via generated token paths resolved in the *calling*
  crate). This also means this crate no longer needs a `version` alongside
  its `git` dependency to pass `cargo publish`, and can be published
  independently of `ddd-toolkit-core`'s publish status.

## [0.2.0] - 2026-07-20

### Fixed

- Removed an invalid `pub use zeroize;` re-export from this proc-macro
  crate's `lib.rs` — proc-macro crates cannot export anything besides their
  proc-macro entry points, so enabling the `zeroize` feature failed to
  compile the crate at all.
- `#[vo(zeroize)]` is now gated behind this crate's own `zeroize` Cargo
  feature. Previously it always emitted a `zeroize::Zeroize::zeroize` call
  regardless of the feature flag, producing a confusing "unresolved crate"
  error for consumers who used the attribute without adding `zeroize` as
  their own direct dependency.
- `ValueObject`/`EntityId`/`SecretVo` now reject a publicly-visible wrapped
  field at derive time. A `pub` field lets external code bypass
  `TryFrom`/`try_new` validation entirely by constructing the type directly;
  the macro can't fix the field, but it can refuse to derive for it.

## [0.1.0] - 2026-07-20

### Added

- `#[derive(ValueObject)]`, `#[derive(EntityId)]`, `#[derive(SecretVo)]`,
  `#[derive(EnumVo)]`, implementing `ddd-toolkit-core`'s corresponding
  traits.
- Unit tests and macro-expansion integration tests for all four derives.

### Fixed

- Several `#[vo(...)]` attributes were previously accepted silently but had
  no effect on macros they weren't wired up for (`validate` and
  `no_display` on `EnumVo`; `rename_all` on
  `ValueObject`/`EntityId`/`SecretVo`) — `validate` in particular was a
  silent validation bypass. `EnumVo` now rejects `validate` (variant
  membership is already exhaustive) and properly supports `no_display`; the
  other three macros now reject `rename_all`.

[Unreleased]: https://github.com/ryoshrimp/ddd-toolkit-macro/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/ryoshrimp/ddd-toolkit-macro/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/ryoshrimp/ddd-toolkit-macro/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/ryoshrimp/ddd-toolkit-macro/releases/tag/v0.1.0
