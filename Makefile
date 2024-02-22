.PHONY: all build test clean

all: build

build:
	@echo Building DEBUG...
	@cargo build

lint:
	@echo Linting...
	@cargo clippy

clean:
	cargo clean

release: lint
	@echo Building RELEASE...
	cargo build --release

update-deps:
	cargo update

