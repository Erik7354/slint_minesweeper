build:
	cargo build

build/wasm:
	wasm-pack build --target web

build/release:
	cargo build --release
	wasm-pack build --release --target web

run:
	cargo run