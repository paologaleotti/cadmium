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

release-lambda:
	@echo Building RELEASE for AWS Lambda...
	RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --bin lambda
	rm -rf target/release/aws_lambda
	mkdir target/release/aws_lambda
	mv target/release/lambda target/release/aws_lambda


update-deps:
	cargo update

