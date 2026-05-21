.DEFAULT_GOAL := help
SHELL := /bin/bash

# Cargo binaries inside the workspace; built once, reused everywhere.
NEW_LESSON   := cargo run --quiet --package new-lesson --
SLIDES_DEV   := cargo run --quiet --package slides-dev --
COMPILE_FAIL := cargo run --quiet --package compile-fails --

.PHONY: help
help: ## Show this help.
	@awk 'BEGIN {FS = ":.*##"; printf "Available targets:\n\n"} \
	      /^[a-zA-Z_-]+:.*##/ { printf "  \033[36m%-22s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)
