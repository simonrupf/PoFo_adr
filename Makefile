.PHONY: build release lint help

build: src/main.rs ## Build the binary for debug (default).
	cargo build

release: src/main.rs ## Build the binary for release.
	cargo build --release

lint: ## Run fmt & clippy on the code to come up with improvements.
	cargo fmt
	cargo clippy --all-targets --all-features -- -D warnings

help: ## Displays these usage instructions.
	@echo "Usage: make <target(s)>"
	@echo
	@echo "Specify one or multiple of the following targets and they will be processed in the given order:"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "%-16s%s\n", $$1, $$2}' $(MAKEFILE_LIST)
