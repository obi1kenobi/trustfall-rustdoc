# Code-generator for `trustfall-rustdoc`

This tool automates the process of adding support for a new rustdoc JSON format version to `trustfall-rustdoc`.

It's intended to primarily be used via `./scripts/add_next_rustdoc_version.sh` from the repo root, which generates code to support format `N + 1` if `N` is the largest previously-supported format version.

This tool also assumes that a corresponding `trustfall-rustdoc-adapter` version for that `N + 1` format has already been created and published to crates.io. Please ensure that is the case, or else the generated code here will fail to compile.
