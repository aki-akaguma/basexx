#

all: readme

readme: README.md

README.md: README.tpl src/lib.rs
	cargo readme > $@

clean:
	@cargo clean
	@rm -f z.* *.log *.tmp

clippy:
	cargo clippy --offline --tests --workspace

clippy-all:
	cargo clippy --tests -- -Dclippy::all

test-build:
	cargo test --no-run
	sync

test:
	cargo test --offline

fmt:
	cargo fmt

doc:
	cargo doc --features dox

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

cov:
	cargo llvm-cov
	cargo llvm-cov --output-dir target/llvm-cov --text report
	cargo llvm-cov --output-dir target/llvm-cov --html report
