#!/bin/bash
set -euo pipefail
NDK=/home/gigli/Android/Sdk/ndk/28.2.13676358
STRIP="$NDK/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-strip"
JN=/mnt/c/Users/gigli/Kotisatama/Katselin/android/apk/servoapp/src/main/jniLibs
ROOT=/mnt/c/Users/gigli/Kotisatama/Katselin-haku
mkdir -p "$JN/x86_64" "$JN/arm64-v8a"
cp "$ROOT/target/x86_64-linux-android/release/meilisearch" "$JN/x86_64/libmeilisearch.so"
cp "$ROOT/target/aarch64-linux-android/release/meilisearch" "$JN/arm64-v8a/libmeilisearch.so"
"$STRIP" "$JN/x86_64/libmeilisearch.so" "$JN/arm64-v8a/libmeilisearch.so"
ls -la "$JN/x86_64/libmeilisearch.so" "$JN/arm64-v8a/libmeilisearch.so"
file "$JN/x86_64/libmeilisearch.so" "$JN/arm64-v8a/libmeilisearch.so"
