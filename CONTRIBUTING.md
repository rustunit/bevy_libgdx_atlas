# Contributing to bevy_libgdx_atlas

We happily welcome folks to contribute in some way. This document will help you in contributing, however you'll do that.

## Reporting issues and requesting features

Please report issues and request features at the relevant sections of this repository. Follow the guidelines in the issue templates for how to fill them out properly.

Check for duplicates before submitting, please.

## Testing

To test `bevy_libgdx_atlas` in the same way that the continuous integration does, run:

`cargo clippy -Dwarnings && cargo test --verbose && cargo doc --all-features --no-deps --document-private-items --keep-going && cargo fmt --check` 

This will check to be sure that your code runs, hasn't broken anything, has adequate documentation, and is formatted to our standards.
