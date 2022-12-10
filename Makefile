
build:
	cargo build --target="riscv32i-unknown-none-elf" --release --bin forth

forth: build
	riscv64-unknown-elf-objcopy -O binary target/riscv32i-unknown-none-elf/release/forth target/riscv32i-unknown-none-elf/release/forth.bin

clean:
	rm -fr target

