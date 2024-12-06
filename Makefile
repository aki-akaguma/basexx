#

all:
	cargo build

clean:
	cargo clean

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
