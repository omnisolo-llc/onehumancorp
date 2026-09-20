.DEFAULT_GOAL := help
CARGO ?= cargo
NPM ?= npm
RUST_TEST_ARGS ?=
E2E_ARGS ?=

.PHONY: help test test-rust test-backend test-node test-contracts test-e2e build-e2e build-web lint lint-rust lint-backend lint-node

help:
	@echo 'make test: all Rust, Node/frontend/CLI, contracts and real-stack E2E tests'
	@echo 'make lint: Rust formatting/Clippy, ESLint and TypeScript checks'
	@echo 'Focused targets: test-rust test-backend test-node test-contracts test-e2e'
	@echo 'See docs/development/native-build.md for dependencies and setup.'

# Recursive recipe lines keep stages ordered and fail fast, even with make -j.
test:
	$(MAKE) test-rust
	$(MAKE) test-node
	$(MAKE) test-contracts
	$(MAKE) test-e2e

test-rust:
	$(CARGO) test --locked --workspace $(RUST_TEST_ARGS)

# A focused headless lane; never substitutes for the full workspace gate.
test-backend:
	$(CARGO) test --locked --workspace --exclude app $(RUST_TEST_ARGS)

test-node:
	$(NPM) run test:scripts
	$(NPM) run test:web
	$(NPM) --prefix src/cli test
	$(NPM) run test:desktop-ui

test-contracts:
	$(NPM) run test:contracts

build-web:
	$(NPM) run build:web

build-e2e:
	$(CARGO) build --locked -p omnisolo -p omnisolo_builtin_agent -p omnisolo_harness_worker --bins
	$(MAKE) build-web

test-e2e:
	$(MAKE) build-e2e
	$(NPM) run test:e2e -- $(E2E_ARGS)

lint:
	$(MAKE) lint-rust
	$(MAKE) lint-node

lint-rust:
	$(CARGO) fmt --all -- --check
	$(CARGO) clippy --locked --workspace --all-targets -- -D warnings

# CI partitions lint by package: this plus desktop Clippy covers the workspace
# without making the desktop job compile the entire backend a second time.
lint-backend:
	$(CARGO) fmt --all -- --check
	$(CARGO) clippy --locked --workspace --exclude app --all-targets -- -D warnings

lint-node:
	$(NPM) run lint:node
	$(NPM) run typecheck:web
	$(NPM) --prefix src/cli run typecheck
