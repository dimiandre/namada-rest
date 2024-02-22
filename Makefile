# Simple Makefile for Rust projects

# Define the default target. When you run `make`, this target will be built.
.PHONY: all
all: build

# Define the build target
.PHONY: build
build:
	@cargo build --release

# Target for running the program (assuming your binary is named after your package in Cargo.toml)
.PHONY: run
run:
	@cargo run

# Target for cleaning up the project (removes target directory and Cargo.lock file)
.PHONY: clean
clean:
	@cargo clean

# Target for running tests
.PHONY: test
test:
	@cargo test
