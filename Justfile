alias c := check
alias t := test
alias d := doc
alias f := format
alias fmt := format

default:
    just --list

ci: build check wasm test doc format

build:
    cargo b --all-targets

check:
    cargo clippy --all-targets --all-features -- -Dwarnings

wasm:
    cargo clippy --target wasm32-unknown-unknown --all-features -- -Dwarnings

test:
    cargo test --all-features

doc:
    cargo doc --all-features --no-deps --document-private-items --keep-going

format:
    cargo fmt --check

example:
    cargo run --example animation