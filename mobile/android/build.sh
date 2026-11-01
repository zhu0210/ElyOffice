#!/usr/bin/env bash
# ElyMark's debug APK: Rust library, stripped, then Gradle.
set -euo pipefail

here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../.." && pwd)
abi=${ELY_ABI:-arm64-v8a}
case $abi in
  arm64-v8a) triple=aarch64-linux-android ;;
  x86_64) triple=x86_64-linux-android ;;
  *) echo "unknown ABI $abi" >&2; exit 2 ;;
esac
: "${ANDROID_NDK_HOME:?set ANDROID_NDK_HOME}"
export RUST_FONTCONFIG_DLOPEN=on
cd "$root"
cargo ndk -t "$abi" -P 26 rustc -p ely-mark --lib --crate-type cdylib
built=$root/target/$triple/debug
# gpui-mobile's own library kinds are never shipped.
rm -f "$built"/deps/libgpui_mobile-*.a "$built"/deps/libgpui_mobile-*.so
libs=$here/app/src/main/jniLibs/$abi
mkdir -p "$libs"
"$ANDROID_NDK_HOME"/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-strip --strip-debug \
  -o "$libs/libely_mark.so" "$built/libely_mark.so"
cd "$here"
./gradlew assembleDebug
echo "apk: $here/app/build/outputs/apk/debug/app-debug.apk"
