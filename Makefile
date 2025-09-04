.PHONY: core cli server web all

core:
	cd core && maturin develop

cli: core
	cd cli && python -m speedtest-cli greet

server:
	cd server && cargo run

web:
	cd web && npm install && npm run dev

all: core cli server web
