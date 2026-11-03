#!/usr/bin/env bash
# 手机图标，取自母版
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
master=$root/assets/icons/ely-mark.svg
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
# 安卓前景，安全区内
res=$root/mobile/android/app/src/main/res
mkdir -p "$res/mipmap-xxxhdpi" "$res/mipmap-anydpi-v26"
rsvg-convert -w 264 -h 264 "$master" -o "$work/fg.png"
magick -size 432x432 xc:none "$work/fg.png" -gravity center -composite \
  "$res/mipmap-xxxhdpi/ic_launcher_foreground.png"
# iOS 不透明底
set=$root/mobile/ios/Assets.xcassets/AppIcon.appiconset
mkdir -p "$set"
rsvg-convert -w 880 -h 880 "$master" -o "$work/ios.png"
magick -size 1024x1024 xc:'#F4F4F2' "$work/ios.png" -gravity center -composite \
  -alpha off "$set/icon-1024.png"
