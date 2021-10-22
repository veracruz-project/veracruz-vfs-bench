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
	mkdir -p target/programs
	mkdir -p target/results
	mkdir -p target/scratch
	$(strip ! mountpoint -q target/scratch \
		&& sudo mount -t ramfs ramfs target/scratch \
		&& sudo chmod a+rwx target/scratch \
		|| true )
	$(strip cp \
		target/wasm32-wasi/release/veracruz-vfs-bench.wasm \
		target/programs )

.PHONY: bench-vc-fee
bench-vc-fee: build
	./bench.py vc-fee target/results-vc-fee.json $(JOBS)

.PHONY: bench-wasmtime
bench-wasmtime: build
	./bench.py wasmtime target/results-wasmtime.json $(JOBS)

.PHONY: bench
bench: bench-vc-fee bench-wasmtime

.PHONY: graph
graph:
	$(strip ./graph.py target/results.svg \
		vc-fee=target/results-vc-fee.json \
		"wasmtime + ramfs"=target/results-wasmtime.json )

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: clean
clean:
	sudo umount target/scratch || true
	cargo clean
	rm -f Cargo.lock

