list:
	@echo "build run"

build:
	reset
	cd py && make
	cargo build -r


run:
	target/release/main