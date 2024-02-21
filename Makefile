.PHONY: all build test clean

all: build

build:
	@echo Started DEBUG build
	@cargo build

clean:
	cargo clean

release:
	@echo Started RELEASE build
	cargo build --release

update-deps:
	cargo update

