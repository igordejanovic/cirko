#!/bin/bash
rm -fr build
mkdir build
TARGETS=(x86_64-unknown-linux-gnu aarch64-apple-darwin x86_64-pc-windows-gnu)
for TARGET in "${TARGETS[@]}"; do
    cargo build --profile release --target "$TARGET"
    if [[ "$TARGET" != *"windows"* ]]; then
        zip -j "build/${TARGET}.zip" "target/$TARGET/release/ћирко"
    else
        zip -j "build/${TARGET}.zip" "target/$TARGET/release/ћирко.exe"
    fi
done
