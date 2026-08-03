#!/usr/bin/env bash
# Katselin-haku M1 — build Meilisearch for Android NDK (bionic).
# Usage (WSL): ./android/build-android.sh [x86_64|aarch64|both]
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

NDK="${ANDROID_NDK_HOME:-${NDK:-$HOME/Android/Sdk/ndk/28.2.13676358}}"
TOOLCHAIN="$NDK/toolchains/llvm/prebuilt/linux-x86_64"
API=29

if [[ ! -x "$TOOLCHAIN/bin/x86_64-linux-android${API}-clang" ]]; then
  echo "NDK clang not found under $TOOLCHAIN" >&2
  exit 1
fi

export PATH="$HOME/.cargo/bin:$PATH"
export CC_x86_64_linux_android="$TOOLCHAIN/bin/x86_64-linux-android${API}-clang"
export AR_x86_64_linux_android="$TOOLCHAIN/bin/llvm-ar"
export CXX_x86_64_linux_android="$TOOLCHAIN/bin/x86_64-linux-android${API}-clang++"
export CC_aarch64_linux_android="$TOOLCHAIN/bin/aarch64-linux-android${API}-clang"
export AR_aarch64_linux_android="$TOOLCHAIN/bin/llvm-ar"
export CXX_aarch64_linux_android="$TOOLCHAIN/bin/aarch64-linux-android${API}-clang++"

# Feature profile (ANDROID-PORT-SUUNNITELMA Vaihe 2, adjusted M1):
# --no-default-features drops mini-dashboard.
# Skip meilisearch-types/all-tokenizations: pulls lindera-ko-dic/unidic
# (Japanese/Korean) whose build scripts fail on Android cross-compile.
# Latin tokenization (Finnish/Swedish) comes from charabia default-features=false baseline.
FEATURES=""
# No lmdb-posix-sem (PoC 4a: ENOSYS on bionic).

TARGETS="${1:-both}"
build_one() {
  local triple="$1"
  echo "=== building meilisearch for $triple ==="
  if [[ -n "$FEATURES" ]]; then
    cargo build \
      --release \
      --package meilisearch \
      --target "$triple" \
      --no-default-features \
      --features "$FEATURES"
  else
    cargo build \
      --release \
      --package meilisearch \
      --target "$triple" \
      --no-default-features
  fi
  local out="target/$triple/release/meilisearch"
  ls -la "$out"
  file "$out"
}

case "$TARGETS" in
  x86_64) build_one x86_64-linux-android ;;
  aarch64|arm64) build_one aarch64-linux-android ;;
  both)
    build_one x86_64-linux-android
    build_one aarch64-linux-android
    ;;
  *)
    echo "usage: $0 [x86_64|aarch64|both]" >&2
    exit 1
    ;;
esac

echo "Done. Copy to Katselin jniLibs as libmeilisearch.so when ready."
