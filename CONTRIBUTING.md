# Contributing to ddd-toolkit-macro

Thanks for your interest in contributing!

## Reporting bugs / requesting features

Please use the issue templates (bug report / feature request) when opening an
issue - they help make sure we get the context needed to act on it quickly.

## Development setup

- Rust 1.85+ (Edition 2024)
- `cargo test` / `cargo fmt` / `cargo clippy` as usual, no special setup required.
- Macro-expansion tests live under `tests/ui/*.rs` and use
  [`trybuild`](https://docs.rs/trybuild); see `tests/compile_fail.rs`.

## Running checks locally

Before opening a PR, run the same checks CI runs (see
[`.github/workflows/ci.yml`](.github/workflows/ci.yml)):

```sh
cargo test
cargo test --features zeroize
cargo test --features serde

cargo fmt --check
cargo clippy --all-targets --features zeroize,serde -- -D warnings
```

## Submitting changes

`main` is protected: direct pushes are disabled and all changes go through a
pull request that must pass CI before merging. This project doesn't require a
mandatory review at this stage (it's solo-maintained), but please describe the
change and the reasoning behind it in the PR description - that's what gets
read during merge.

## License

By contributing, you agree that your contributions will be dual-licensed
under the [MIT](LICENSE-MIT) and [Apache-2.0](LICENSE-APACHE) licenses, same
as the rest of this project.
