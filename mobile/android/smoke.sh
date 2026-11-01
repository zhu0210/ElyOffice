#!/usr/bin/env bash
# Emulator smoke: install, launch, screenshot, logs; the window must open.
set -uo pipefail
apk=mobile/android/app/build/outputs/apk/debug/app-debug.apk
mkdir -p smoke
adb install -r "$apk"
adb logcat -c
adb shell am start -n com.elyoffice.mark/dev.gpui.mobile.GpuiActivity
sleep 40
adb exec-out screencap -p > smoke/screenshot.png
adb logcat -d > smoke/logcat.txt
pid=$(adb shell pidof com.elyoffice.mark | tr -d '\r')
if [ -z "$pid" ] || ! grep -q "ElyMark window open" smoke/logcat.txt; then
  echo "::error::ElyMark did not open its window"
  grep -E "ely-mark|FATAL|DEBUG" smoke/logcat.txt | tail -40
  exit 1
fi
echo "ElyMark is up as $pid"
