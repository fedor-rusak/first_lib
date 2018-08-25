clean:
	cargo clean
build:
	cargo build
test:
	cargo test -- --nocapture
doc:
	cargo rustdoc