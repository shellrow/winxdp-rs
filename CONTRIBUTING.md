# Contributing

Bug reports should include the crate/XDP versions, Windows build, NIC and driver,
bind mode, queue, enabled extensions and the exact error. Remove addresses,
credentials and other environment-specific information from attached logs.

## Build checks

Use Windows x64 with MSVC, a Windows SDK and Rust 1.88 or later. The ordinary tests
do not require an installed XDP driver.

```powershell
python -X utf8 tools/check_coverage.py
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo test --workspace --all-features
cargo build --workspace --all-features --examples
$env:RUSTDOCFLAGS = '-D warnings'
cargo doc --workspace --all-features --no-deps
cargo +1.88.0 check --workspace --all-targets --all-features --locked
```

Install the 1.88.0 toolchain before running the last command.
Use [hardware tests](docs/testing.md) for driver-backed checks. Keep local test
logs, signing material and machine-specific setup outside the repository.

## Binding maintenance

Consumer builds use checked-in bindings and vendored headers. Regeneration needs
LLVM/libclang, Python and the MSVC/Windows SDK headers. Set `LIBCLANG_PATH` to the
LLVM library directory. Review the upstream revision and license before updating
headers or their hashes.

```powershell
python -X utf8 tools/bootstrap.py
cargo run --locked --manifest-path tools/generate/Cargo.toml
python -X utf8 tools/coverage.py
cargo fmt --all
python -X utf8 tools/check_coverage.py
cargo test --workspace --all-features
```

`bootstrap.py` generates the C forwarding functions and header-hash manifest.
`tools/generate` generates Rust declarations. `coverage.py` regenerates ABI
comparisons and the API inventory. Review generated changes alongside the upstream
header diff; a matching hash alone is not an ABI review.

Preserve the buffer ownership and cancellation contracts described in
[safety](docs/safety.md). Add regression tests for changes to those contracts.
