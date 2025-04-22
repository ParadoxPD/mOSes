.PHONY: build run debug clean test all default

KERNEL=target/x86_64-unknown-none/debug/bootimage-mOSes.bin
PROJECT_DIR=/home/paradox/Documents/Projects/mOSes

default: run

run: build
	qemu-system-x86_64 -drive format=raw,file=$(KERNEL)

debug: build
	qemu-system-x86_64 -drive format=raw,file=$(KERNEL) -s -S

build:
	CARGO_MANIFEST_DIR=$(PROJECT_DIR) cargo bootimage

clean:
	cargo clean
