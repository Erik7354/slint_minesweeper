build:
	cargo build

build/wasm:
	wasm-pack build --release --target web

run:
	cargo run