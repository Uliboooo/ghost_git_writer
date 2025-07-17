#!/bin/bash

ver="$1"

success_builds=0
success_zips=0

# build
if cargo build --release; then
    success_builds=$((success_builds + 1))
else
    echo "❌ failed to build for arm mac"
fi

if cargo build --release --target x86_64-pc-windows-gnu; then
    success_builds=$((success_builds + 1))
else
    echo "❌ failed to build for windows"
fi

# if cargo build --release --target x86_64-unknown-linux-gnu; then
#     success_builds=$((success_builds + 1))
# else
#     echo "❌ failed to build for linux"
# fi

# zip
mkdir -p ./release

if zip -j ./release/ggw_${ver}_arm_mac.zip ./target/release/ggw; then
    success_zips=$((success_zips + 1))
else
    echo "❌ failed to zip for arm mac"
fi

if zip -j ./release/ggw_${ver}_win.zip ./target/x86_64-pc-windows-gnu/release/ggw.exe; then
    success_zips=$((success_zips + 1))
else
    echo "❌ failed to zip for windows"
fi

# if zip -j ./release/ggw_${ver}_linux.zip ./target/x86_64-unknown-linux-gnu/release/ggw; then
#     success_zips=$((success_zips + 1))
# else
#     echo "❌ failed to zip for linux"
# fi

# build results
[ "$success_builds" -ge 1 ] && echo "✅ success arm mac build"
[ "$success_builds" -ge 2 ] && echo "✅ success windows build"
[ "$success_builds" -ge 3 ] && echo "✅ success linux build"

# zip results
[ "$success_zips" -ge 1 ] && echo "✅ success arm mac zip"
[ "$success_zips" -ge 2 ] && echo "✅ success windows zip"
[ "$success_zips" -ge 3 ] && echo "✅ success linux zip"

