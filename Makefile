clippy:
	@cargo clippy

run:
	@cargo run

check:
	@cargo check
        
fmt:
	@cargo +nightly fmt

test:
	@cargo test --tests -- --nocapture
