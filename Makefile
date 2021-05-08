.PHONY: check build test reader api

check:
	cargo check

build:
	cargo build

test:
	cargo build

reader:
	cargo run --bin reader

api:
	cargo run --bin api
