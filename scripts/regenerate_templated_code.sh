#!/usr/bin/env bash

# Script requirements:
# - python
# - yq

# Usage:
# - `./regenerate_templated_code.sh` will re-run the templated generation process for
#   all the currently-specified rustdoc version numbers in the default features list.

# Preconditions:
# - all supported format version numbers are enabled as default features
# - the list of default features is sorted in increasing order

# Fail on first error, on undefined variables, and on failures in pipelines.
set -euxo pipefail

# Go to the repo root directory.
cd "$(git rev-parse --show-toplevel)"

CURRENT_VERSIONS="$(yq '.features.default[] | sub("v(\d+)", "${1}")' Cargo.toml -r)"

# Generate the new Rust source for the specified versions.
pushd crates/generator
cargo run -- $CURRENT_VERSIONS
popd

# Reformat the generated Rust source code.
cargo fmt

# Ensure cargo regenerates the lockfile.
cargo check
