# Contributing to compress-json-rs

Thank you for your interest in contributing! This document outlines the development workflow and release process.

## Development Setup

```bash
# Clone the repository
git clone https://github.com/web-mech/compress-json-rs.git
cd compress-json-rs

# Setup development environment
make setup

# Run all checks
make check
```

## Making Changes

### Commit Message Convention

We use [Conventional Commits](https://www.conventionalcommits.org/) for automatic version bumping:

| Prefix | Description | Version Bump |
|--------|-------------|--------------|
| `feat:` | New feature | Minor (0.x.0) |
| `fix:` | Bug fix | Patch (0.0.x) |
| `docs:` | Documentation only | Patch |
| `chore:` | Maintenance | Patch |
| `refactor:` | Code refactoring | Patch |
| `test:` | Adding tests | Patch |
| `perf:` | Performance improvement | Patch |
| `BREAKING CHANGE:` or `feat!:` | Breaking change | Major (x.0.0) |

### Examples

```bash
# Feature (minor bump)
git commit -m "feat: add support for custom encoders"

# Bug fix (patch bump)
git commit -m "fix: handle empty arrays correctly"

# Breaking change (major bump)
git commit -m "feat!: change Compressed type to use Vec<u8>"
# or
git commit -m "feat: new API

BREAKING CHANGE: The compress() function now returns Result"
```

## Pull Request Workflow

1. **Create a branch** from `main`
   ```bash
   git checkout -b feature/my-feature
   ```

2. **Make changes** and commit with conventional commit messages

3. **Run checks locally**
   ```bash
   make check  # Runs fmt, lint, and tests
   ```

4. **Push and create PR**
   ```bash
   git push origin feature/my-feature
   ```

5. **CI runs automatically**:
   - ✅ Code formatting check
   - ✅ Clippy linting
   - ✅ Tests on stable, beta, nightly Rust
   - ✅ Documentation build
   - ✅ Release dry-run preview

6. **Review and merge** - Release happens automatically!

## Release Process

### Automatic (Recommended)

Releases happen automatically when PRs are merged to `main`:

1. CI analyzes commits since last release
2. Determines version bump (major/minor/patch)
3. Updates `Cargo.toml`
4. Creates git tag
5. Publishes to crates.io
6. Creates GitHub Release

### Manual Release

If needed, you can trigger a manual release:

```bash
# Preview what would happen
make release-dry-run

# Release with auto-detected bump
make release

# Force specific version bump
make release-major
make release-minor
make release-patch
```

### Manual Publish Only

```bash
# Dry run
make publish-dry-run

# Actual publish
make publish
```

## GitHub Actions Secrets

For releases to work, the following secrets must be configured in the repository:

| Secret | Description | How to Get |
|--------|-------------|------------|
| `CARGO_REGISTRY_TOKEN` | crates.io API token | [crates.io/settings/tokens](https://crates.io/settings/tokens) |

### Setting Up Secrets

1. Go to your repository Settings → Secrets and variables → Actions
2. Click "New repository secret"
3. Add `CARGO_REGISTRY_TOKEN` with your crates.io token

## Local Development Commands

```bash
# Build
make build              # Debug build
make build-release      # Release build

# Test
make test               # Run all tests
make test-verbose       # Run tests with output

# Code Quality
make fmt                # Format code
make lint               # Run clippy
make check              # All checks

# Documentation
make doc                # Build and open docs
make doc-build          # Build docs only

# Version Management
make version            # Show current version
make changelog          # Generate changelog
make bump               # Auto-bump version
make bump-patch         # Bump patch version

# Release
make release-dry-run    # Preview release
make release            # Full release
```

## Project Structure

```
compress-json-rs/
├── .github/
│   └── workflows/
│       ├── ci.yml          # PR checks & dry-run
│       └── release.yml     # Auto-release on merge
├── scripts/
│   ├── analyze-commits.sh  # Determine version bump
│   ├── bump-version.sh     # Update Cargo.toml
│   ├── changelog.sh        # Generate changelog
│   ├── create-release-tag.sh
│   ├── get-version.sh
│   ├── publish.sh
│   └── release.sh          # Full release orchestration
├── src/
│   ├── lib.rs              # Crate root & docs
│   ├── core.rs             # compress/decompress
│   ├── memory.rs           # Compression state
│   ├── encode.rs           # Value encoding
│   ├── number.rs           # Base-62 encoding
│   ├── boolean.rs          # Boolean encoding
│   ├── helpers.rs          # Utility functions
│   ├── config.rs           # Configuration
│   └── debug.rs            # Error handling
├── tests/
│   ├── core_test.rs        # Core functionality tests
│   ├── number_test.rs      # Number encoding tests
│   ├── helpers_test.rs     # Helper function tests
│   └── sample.rs           # Test data
├── Cargo.toml
├── Makefile
└── README.md
```

## Questions?

Feel free to open an issue for any questions or discussions!
