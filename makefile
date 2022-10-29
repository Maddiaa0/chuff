check:
	cargo watch -x check

test:
	cargo watch -x test

fmt:
	cargo +nightly fmt

clippy:
	cargo +nightly clippy

run:
	cargo run 

runp:
	cargo run --release