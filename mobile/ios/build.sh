#!/usr/bin/env bash
# ElyMark's static library for iOS. ELY_SIM=1 builds for the simulator.
set -euo pipefail
root=$(cd "$(dirname "$0")/../.." && pwd)
triple=aarch64-apple-ios; release=
if [ "${ELY_SIM:-}" = 1 ]; then triple=aarch64-apple-ios-sim; fi
# macOS bash 3.2 refuses empty arrays under set -u.
if [ "${ELY_RELEASE:-}" = 1 ]; then release=--release; fi
cd "$root"
rustup target add "$triple"
cargo rustc -p ely-mark --lib --crate-type staticlib --target "$triple" $release
