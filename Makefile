all: test

build:
	cargo build --release

test: build
	./test.sh
