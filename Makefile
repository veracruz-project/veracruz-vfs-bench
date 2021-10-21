# A Makefile
#
# AUTHORS
#
# The Veracruz Development Team.
#
# COPYRIGHT AND LICENSING
#
# See the `LICENSING.markdown` file in the Veracruz root directory for
# licensing and copyright information.

.PHONY: all
all build:
	cargo build --target wasm32-wasi --release
	mkdir -p target/vc-fee/programs
	mkdir -p target/vc-fee/results
	mkdir -p target/vc-fee/scratch
	$(strip cp \
		target/wasm32-wasi/release/veracruz-vfs-bench.wasm \
		target/vc-fee/programs )

.PHONY: bench
bench: build
	./bench.py target/results.json $(JOBS)

.PHONY: graph
graph:
	./graph.py target/results.json target/results.svg

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: clean
clean:
	cargo clean
	rm -f Cargo.lock

