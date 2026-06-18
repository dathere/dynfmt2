# Task Completion Checklist

Run these before considering a code task done (mirrors `.github/workflows/ci.yml`):

1. `cargo fmt --all -- --check`
2. `cargo clippy --all-features --tests -- -D clippy::all`
3. `cargo test --all-features`
4. `cargo test --no-default-features`

All four must pass. CI sets `RUSTFLAGS=-Dwarnings`, so any warning is a failure. After editing
parsing or error messages, expect to update `tests/test_error_messages.rs` (it asserts exact
error text).