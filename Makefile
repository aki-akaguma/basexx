#

all:
	cargo build

clean:
	cargo clean

clippy:
	cargo clippy --tests

clippy-all:
	cargo clippy --tests -- -Dclippy::all

test-build:
	cargo test --no-run
	sync

test:
	cargo test

bench-build:
	cargo bench --features="abench" --no-run
	sync

bench:
	cargo bench --features="abench"

ubench-build:
	cargo bench --features="ubench" --no-run
	sync

ubench:
	cargo bench --features="ubench"
