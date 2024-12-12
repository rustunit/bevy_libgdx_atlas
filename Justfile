alias c := check
alias t := test
alias d := doc
alias f := format
alias fmt := format

default:
    just --list

ci: build check test doc format

build:
    cargo b --all-targets

check:
    cargo clippy --all-targets -- -Dwarnings

test:
    cargo test

doc:
    cargo doc --all-features --no-deps --document-private-items --keep-going

format:
    cargo fmt --check

example:
    cargo run --example animation