.PHONY: all build release exec install

all: build

build:
	cargo build

release:
	cargo build --release

exec:
	cargo run --bin mdparser

install: release
	cp target/release/mdparser /usr/local/bin/mdparser
