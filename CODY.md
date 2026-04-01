# CODY.md

This file provides guidance to Cody Code when working with code in this repository.

## Detected stack
- Languages: Rust.
- Frameworks: none detected from the supported starter markers.

## Verification
- Run Rust verification from `rust/`: `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`

## Repository shape
- `rust/` contains the Rust workspace and active CLI/runtime implementation.

## Working agreement
- Prefer small, reviewable changes and keep generated bootstrap files aligned with actual repo workflows.
- Keep shared defaults in `.cody.json`; reserve `.cody/settings.local.json` for machine-local overrides.
- Do not overwrite existing `CODY.md` content automatically; update it intentionally when repo workflows change.
