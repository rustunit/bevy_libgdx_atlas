ci: check test doc format

check:
    cargo clippy --verbose -- -Dwarnings

test:
    cargo test --verbose

doc:
    cargo doc --all-features --no-deps --document-private-items --keep-going

format:
    cargo fmt --check
