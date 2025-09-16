RUSTFLAGS_COMMON=

all:
	cargo build --manifest-path /workspace/time_userland/Cargo.toml

clean:
	cargo clean --manifest-path /workspace/time_userland/Cargo.toml

llvm:
	cargo rustc -p aster-time-ffi --manifest-path /workspace/time_userland/Cargo.toml -- --emit=llvm-ir,llvm-bc

