#!/usr/bin/env bash

# Script requirements:
# - python
# - yq

# Usage:
# - `./add_new_rustdoc_version.sh <number>` to add rustdoc JSON format number explicitly
# - omit `<number>` to add format `N + 1` if `N` is the current maximum supported format number.

# Preconditions:
# - a `trustfall-rustdoc-adapter` for the specified new rustdoc format must already be available.

NEXT_VERSION_NUMBER="$1"

# Fail on first error, on undefined variables, and on failures in pipelines.
set -euo pipefail

# Go to the repo root directory.
cd "$(git rev-parse --show-toplevel)"

CURRENT_VERSIONS="$(yq '.features.default' Cargo.toml -o json | \
    python -m json.tool --compact | \
    sed 's/,/, /g' | \
    sed 's/\[//g' | \
    sed 's/]//g')"

if [[ "$NEXT_VERSION_NUMBER" == "" ]]; then
    NEXT_VERSION_NUMBER="$(yq '.features.default.[-1] | sub("v(\d+)", "${1}") | to_number | (. + 1)' Cargo.toml -o json -r)"
fi

ALL_VERSIONS="$(yq '.features.default[] | sub("v(\d+)", "${1}")' Cargo.toml -o json -r) ${NEXT_VERSION_NUMBER}"

# Generate the new Rust source for the specified versions.
pushd crates/generator
cargo run -- $ALL_VERSIONS
popd

# Reformat the generated Rust source code.
cargo fmt

# Update the Cargo.toml file to add the new dependency and feature number.

# '1h;2,$H;$!d;g' means "look two lines at a time":
# https://unix.stackexchange.com/questions/26284/how-can-i-use-sed-to-replace-a-multi-line-string
sed -e '1h;2,$H;$!d;g' \
    -e "s/\n\[features\]/trustfall-rustdoc-adapter-v${NEXT_VERSION_NUMBER} = { package = \"trustfall-rustdoc-adapter\", version = \">=${NEXT_VERSION_NUMBER}.0.0,<${NEXT_VERSION_NUMBER}.1.0\", optional = true }\n\n[features]/" \
    -i Cargo.toml

DEFAULT_MATCHER="default = \[${CURRENT_VERSIONS}\]"
sed -e "s/$DEFAULT_MATCHER/default = [${CURRENT_VERSIONS}, \"v${NEXT_VERSION_NUMBER}\"]/" \
    -i Cargo.toml

echo "v${NEXT_VERSION_NUMBER} = [\"dep:trustfall-rustdoc-adapter-v${NEXT_VERSION_NUMBER}\"]" >>Cargo.toml

# Ensure cargo regenerates the lockfile.
cargo check
