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

VARIABLE ?= "sizes"

.PHONY: all
all build:
	cargo build --target wasm32-wasi --release
	mkdir -p target/programs
	mkdir -p target/results
	mkdir -p target/scratch
	$(strip ! mountpoint -q target/scratch \
		&& sudo mount -t tmpfs -o size=32g tmpfs target/scratch \
		&& sudo chmod a+rwx target/scratch )
	$(strip cp \
		target/wasm32-wasi/release/veracruz-vfs-bench.wasm \
		target/programs )

.PHONY: bench-vc-fee
bench-vc-fee: build
	./bench-$(VARIABLE).py vc-fee target/results-vc-fee-$(VARIABLE).json $(JOBS)

.PHONY: bench-wasmtime
bench-wasmtime: build
	./bench-$(VARIABLE).py wasmtime target/results-wasmtime-$(VARIABLE).json $(JOBS)

.PHONY: bench
bench: bench-vc-fee bench-wasmtime

.PHONY: graph
graph:
	$(strip ./graph-$(VARIABLE).py results/results-$(VARIABLE).svg \
		vc-fee=target/results-vc-fee-$(VARIABLE).json \
		"wasmtime + tmpfs"=target/results-wasmtime-$(VARIABLE).json )

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: clean
clean:
	sudo umount target/scratch || true
	cargo clean
	rm -f Cargo.lock

