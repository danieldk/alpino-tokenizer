#!/bin/bash

set -ex

# Fail early on formatting errors.
if [ "$TRAVIS_RUST_VERSION" = "stable" ]; then
  cargo fmt --all -- --check
fi

cargo check
cargo test

if [ "$TRAVIS_RUST_VERSION" = "stable" ]; then
  cargo clippy -- -D warnings
fi


