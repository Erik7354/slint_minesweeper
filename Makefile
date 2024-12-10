default: build/all/release

build:
	cargo build

build/wasm:
	wasm-pack build --target web

build/all: build build/wasm

build/all/release:
	cargo build --release
	wasm-pack build --release --target web

run:
	cargo run