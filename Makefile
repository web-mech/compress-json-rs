# compress-json-rs Makefile
# Local project CLI scaffolding for development and release management

.PHONY: help build test clean fmt lint doc check install \
        release release-major release-minor release-patch release-dry-run \
        publish publish-dry-run version bump changelog

# Default target
.DEFAULT_GOAL := help

# Colors for terminal output
BLUE := \033[34m
GREEN := \033[32m
YELLOW := \033[33m
RED := \033[31m
RESET := \033[0m

#==============================================================================
# HELP
#==============================================================================

help: ## Show this help message
	@echo ""
	@echo "$(BLUE)compress-json-rs$(RESET) - Development and Release Commands"
	@echo ""
	@echo "$(GREEN)Development:$(RESET)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | grep -E "(build|test|clean|fmt|lint|doc|check)" | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(YELLOW)%-18s$(RESET) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(GREEN)Release:$(RESET)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | grep -E "(release|publish|version|bump|changelog)" | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(YELLOW)%-18s$(RESET) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(GREEN)Examples:$(RESET)"
	@echo "  make test              Run all tests"
	@echo "  make release           Auto-bump and release"
	@echo "  make release-minor     Release with minor version bump"
	@echo "  make release-dry-run   Preview release without changes"
	@echo ""

#==============================================================================
# DEVELOPMENT
#==============================================================================

build: ## Build the project
	@echo "$(BLUE)Building...$(RESET)"
	cargo build

build-release: ## Build optimized release binary
	@echo "$(BLUE)Building release...$(RESET)"
	cargo build --release

test: ## Run all tests
	@echo "$(BLUE)Running tests...$(RESET)"
	cargo test

test-verbose: ## Run tests with output
	@echo "$(BLUE)Running tests (verbose)...$(RESET)"
	cargo test -- --nocapture

clean: ## Clean build artifacts
	@echo "$(BLUE)Cleaning...$(RESET)"
	cargo clean
	rm -rf target/doc

fmt: ## Format code with rustfmt
	@echo "$(BLUE)Formatting code...$(RESET)"
	cargo fmt

fmt-check: ## Check code formatting
	@echo "$(BLUE)Checking formatting...$(RESET)"
	cargo fmt --check

lint: ## Run clippy linter
	@echo "$(BLUE)Running clippy...$(RESET)"
	cargo clippy -- -D warnings

doc: ## Generate documentation
	@echo "$(BLUE)Generating documentation...$(RESET)"
	cargo doc --no-deps --open

doc-build: ## Build documentation without opening
	@echo "$(BLUE)Building documentation...$(RESET)"
	cargo doc --no-deps

check: fmt-check lint test ## Run all checks (format, lint, test)
	@echo "$(GREEN)All checks passed!$(RESET)"

install: ## Install locally
	@echo "$(BLUE)Installing locally...$(RESET)"
	cargo install --path .

#==============================================================================
# VERSION MANAGEMENT
#==============================================================================

version: ## Show current version
	@./scripts/get-version.sh

bump: ## Auto-bump version based on commits
	@./scripts/bump-version.sh auto

bump-major: ## Bump major version
	@./scripts/bump-version.sh major

bump-minor: ## Bump minor version
	@./scripts/bump-version.sh minor

bump-patch: ## Bump patch version
	@./scripts/bump-version.sh patch

changelog: ## Generate changelog from commits
	@./scripts/changelog.sh

#==============================================================================
# RELEASE
#==============================================================================

release: check ## Full release: auto-bump, tag, push, publish
	@echo "$(BLUE)Starting release process...$(RESET)"
	@./scripts/release.sh auto

release-major: check ## Release with major version bump
	@echo "$(BLUE)Starting major release...$(RESET)"
	@./scripts/release.sh major

release-minor: check ## Release with minor version bump
	@echo "$(BLUE)Starting minor release...$(RESET)"
	@./scripts/release.sh minor

release-patch: check ## Release with patch version bump
	@echo "$(BLUE)Starting patch release...$(RESET)"
	@./scripts/release.sh patch

release-dry-run: ## Preview release without making changes
	@echo "$(BLUE)Release dry run...$(RESET)"
	@./scripts/release.sh --dry-run

release-local: check ## Release locally (no push, no publish)
	@echo "$(BLUE)Local release...$(RESET)"
	@./scripts/release.sh --no-push --skip-publish

#==============================================================================
# PUBLISHING
#==============================================================================

publish: ## Publish to crates.io
	@echo "$(BLUE)Publishing to crates.io...$(RESET)"
	@./scripts/publish.sh

publish-dry-run: ## Dry run publish to crates.io
	@echo "$(BLUE)Publish dry run...$(RESET)"
	@./scripts/publish.sh --dry-run

#==============================================================================
# UTILITIES
#==============================================================================

setup: ## Setup development environment
	@echo "$(BLUE)Setting up development environment...$(RESET)"
	@chmod +x scripts/*.sh
	@rustup component add rustfmt clippy
	@echo "$(GREEN)Setup complete!$(RESET)"

ci: ## Run CI checks (used in GitHub Actions)
	@echo "$(BLUE)Running CI checks...$(RESET)"
	cargo fmt --check
	cargo clippy -- -D warnings
	cargo test
	cargo doc --no-deps
	@echo "$(GREEN)CI checks passed!$(RESET)"
