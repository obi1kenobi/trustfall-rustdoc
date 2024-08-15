# Code-generator for `trustfall-rustdoc`

This tool automates the process of adding support for a new rustdoc JSON format version to `trustfall-rustdoc`.

It's intended to primarily be used via `./scripts/add_new_rustdoc_version.sh` from the repo root.

This tool also assumes that a corresponding `trustfall-rustdoc-adapter` version for the specified formats has already been created and published to crates.io. Please ensure that is the case, or else the generated code here will fail to compile.
