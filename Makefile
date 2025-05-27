all: test

build:
	cargo build --release

test: build
	./target/release/rcc "3+5-2" > tmp.s
	riscv-none-elf-gcc -march=rv64gc -mabi=lp64 -o tmp.elf tmp.s
	qemu-riscv64 ./tmp.elf
