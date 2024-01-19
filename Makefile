list:
	@echo "build"

build:
	cd py && make
	cargo build -r