#!/usr/bin/env bash
set -euo pipefail

# Build the target as LLVM IR and instrument with DataFlowSanitizer (dfsan)

if ! command -v opt >/dev/null; then
  echo "opt not found in PATH" >&2
  exit 1
fi
if ! command -v clang >/dev/null; then
  echo "clang not found in PATH" >&2
  exit 1
fi

PKG=${1:-time_demo}
BIN=${2:-time_demo}
MODE=${3:-debug}

if [ -f /usr/local/cargo/env ]; then
  source /usr/local/cargo/env
fi

# Emit LLVM IR for the binary target
RUSTFLAGS="--emit=llvm-ir" cargo rustc -p "$PKG" --bin "$BIN" -- -C debuginfo=1

# Locate the produced IR for the binary under deps
TARGET_DIR="target/${MODE}/deps"
mapfile -t BCS < <(find "$TARGET_DIR" -maxdepth 1 -type f -name "${BIN}-*.ll" 2>/dev/null || true)

if [ ${#BCS[@]} -eq 0 ]; then
  echo "No LLVM IR found for time_demo under $TARGET_DIR" >&2
  exit 1
fi

IR=${BCS[0]}
OUT_LL=${IR%.ll}.dfsan.ll

echo "Instrumenting $IR -> $OUT_LL"
opt -passes=dfsan --dfsan-abilist=/dev/null -S "$IR" -o "$OUT_LL"

echo "Instrumented IR at: $OUT_LL"

