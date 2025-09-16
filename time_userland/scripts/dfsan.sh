#!/usr/bin/env bash
set -euo pipefail

ROOT=/workspace/time_userland
cd "$ROOT"

# Build LLVM bitcode for the FFI crate
cargo rustc -p aster-time-ffi -- --emit=llvm-bc

BC=$(find target -name "libaster_time_ffi*.bc" | head -n1)
if [ -z "$BC" ]; then echo "bitcode not found"; exit 1; fi

# Instrument with DataFlowSanitizer
opt -passes=dfsan -dfsan-args-abi -dfsan-share-abi -o ${BC%.bc}.dfsan.bc "$BC"

# Link to executable test using clang with dfsan runtime
clang ${BC%.bc}.dfsan.bc -fsanitize=dataflow -lstdc++ -lm -ldl -lpthread -o target/dfsan_app || true

echo "Instrumented bitcode: ${BC%.bc}.dfsan.bc"
echo "Executable (if linked): target/dfsan_app"

