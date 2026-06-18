# Suggested Commands

## Build
- `cargo build`
- `cargo build --all-features`

## Test (CI runs BOTH feature configurations)
- `cargo test --all-features`
- `cargo test --no-default-features`   # json off → python `%r` errors
- Single test: `cargo test --all-features test_width_formatting`
- Single test file: `cargo test --all-features --test test_python`
  (test files: `test_python`, `test_curly`, `test_error_messages`)

## Lint / format (CI runs with RUSTFLAGS=-Dwarnings; warnings fail)
- `cargo fmt --all -- --check`
- `cargo clippy --all-features --tests -- -D clippy::all`

## CI gate
`.github/workflows/ci.yml` has 3 jobs: (1) fmt + clippy, (2) `test --all-features`,
(3) `test --no-default-features`. A change must pass all three.

## System notes (Darwin/macOS)
Standard unix tools behave normally here. No project-specific Darwin quirks. `git` is the VCS;
default/main branch is `master`.