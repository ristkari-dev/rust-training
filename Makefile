.DEFAULT_GOAL := help
SHELL := /bin/bash

# Cargo binaries inside the workspace; built once, reused everywhere.
NEW_LESSON   := cargo run --quiet --package new-lesson --
SLIDES_DEV   := cargo run --quiet --package slides-dev --
COMPILE_FAIL := cargo run --quiet --package compile-fails --
BUILD_INDEX  := cargo run --quiet --package build-index --

.PHONY: help
help: ## Show this help.
	@awk 'BEGIN {FS = ":.*##"; printf "Available targets:\n\n"} \
	      /^[a-zA-Z_-]+:.*##/ { printf "  \033[36m%-22s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

.PHONY: build
build: ## Compile every crate in the workspace, including exercise stubs.
	cargo build --workspace --all-targets

.PHONY: test
test: build ## CI test: tools + solutions pass; exercises ship broken.
	cargo test
	$(COMPILE_FAIL) --expect broken lessons

.PHONY: lint
lint: ## Run clippy and rustfmt --check across the workspace.
	cargo clippy --workspace --all-targets -- -D warnings
	cargo fmt --all --check

.PHONY: fmt
fmt: ## Format the workspace.
	cargo fmt --all

.PHONY: ci
ci: lint test ## Run the full local CI sequence.

.PHONY: new-lesson
new-lesson: ## Scaffold a new lesson. Usage: make new-lesson NAME=NN-name
	@if [ -z "$(NAME)" ]; then echo "NAME is required (e.g. make new-lesson NAME=07-ownership)"; exit 2; fi
	$(NEW_LESSON) $(NAME)

.PHONY: slides-dev
slides-dev: ## Serve a lesson's slides on http://localhost:8000. Usage: make slides-dev LESSON=NN-name
	@if [ -z "$(LESSON)" ]; then echo "LESSON is required (e.g. make slides-dev LESSON=01-hello-rust)"; exit 2; fi
	$(SLIDES_DEV) --lesson $(LESSON)

.PHONY: verify
verify: ## Student check: run a lesson's exercise tests + compile-fail compiles. Usage: make verify LESSON=NN-name
	@if [ -z "$(LESSON)" ]; then echo "LESSON is required (e.g. make verify LESSON=01-hello-rust)"; exit 2; fi
	cargo test --manifest-path lessons/$(LESSON)/exercises/Cargo.toml
	@if [ -d "lessons/$(LESSON)/exercises/compile_fails" ]; then \
		$(COMPILE_FAIL) --expect compiles lessons/$(LESSON); \
	fi

.PHONY: slides-build
slides-build: ## Build the static slides site into dist/
	$(BUILD_INDEX) --lessons lessons --shared shared/reveal --out dist

.PHONY: slides-docker
slides-docker: ## Build the deploy image and run it locally on http://localhost:8080
	docker build -t rust-training-slides:local -f deploy/Dockerfile .
	@echo "starting container on http://localhost:8080  (Ctrl-C to stop)"
	docker run --rm -p 8080:8080 -e PORT=8080 rust-training-slides:local
