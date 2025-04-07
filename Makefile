default: build/release

build/css:
	npx tailwindcss -i index.css -o tailwind.css --minify

build:
	cargo build

build/web: build/css
	wasm-pack build --target web

build/release: build/css
	cargo build --release
	wasm-pack build --release --target web

run:
	cargo run

run/web: build/web
	caddy run --config Caddyfile