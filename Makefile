help:
	@echo make install
	@echo make list-modules
	@echo make list-dependencies
	@echo ""
	@echo make build
	@echo make format
	@echo make run
	@echo make test
	@echo make clean

install: rustup-install cargo-install

rustup-install:
	rustup component add rustfmt

cargo-install:
	cargo install cargo-modules

list-modules:
	cargo-modules structure

list-dependencies:
	cargo-modules dependencies

build:
	cargo build

format:
	cargo fmt

run:
	cargo run

test:
	cargo test

clean:
	rm -fr ./target
