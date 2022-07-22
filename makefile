run:
	cargo build --release
	rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/rabbit_os -O binary target/riscv64gc-unknown-none-elf/release/rabbit_os.bin