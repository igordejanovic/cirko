#!/bin/bash
rm -fr build
mkdir build
TARGETS=(x86_64-unknown-linux-gnu aarch64-apple-darwin x86_64-pc-windows-gnu)
RELEASE=`git describe --tags --abbrev=0`
for TARGET in "${TARGETS[@]}"; do
    cargo build --profile release --target "$TARGET"
    OUTPUT="build/${TARGET}-${RELEASE}.zip"
    if [[ "$TARGET" != *"windows"* ]]; then
        zip -j "$OUTPUT" "target/$TARGET/release/ћирко"
    else
        zip -j "$OUTPUT" "target/$TARGET/release/ћирко.exe"
    fi
    gpg --armor --detach-sign "$OUTPUT"
done
