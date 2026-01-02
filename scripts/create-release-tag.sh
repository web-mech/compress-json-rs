#!/usr/bin/env bash
# Create a release tag with changelog
#
# Usage: ./scripts/create-release-tag.sh [version]
#        ./scripts/create-release-tag.sh  # Uses version from Cargo.toml

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

# Get version
VERSION="${1:-$("$SCRIPT_DIR/get-version.sh")}"
TAG_NAME="v$VERSION"

echo "Creating release tag: $TAG_NAME"

# Check if tag already exists
if git rev-parse "$TAG_NAME" >/dev/null 2>&1; then
    echo "Error: Tag $TAG_NAME already exists"
    exit 1
fi

# Get commits since last tag for changelog
LAST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "")
if [ -z "$LAST_TAG" ]; then
    CHANGELOG=$(git log --pretty=format:"- %s (%h)" 2>/dev/null | head -50)
else
    CHANGELOG=$(git log "${LAST_TAG}..HEAD" --pretty=format:"- %s (%h)" 2>/dev/null)
fi

# Create tag message
TAG_MESSAGE="Release $VERSION

## Changes

$CHANGELOG

## Installation

\`\`\`toml
[dependencies]
compress-json-rs = \"$VERSION\"
\`\`\`
"

# Create annotated tag
git tag -a "$TAG_NAME" -m "$TAG_MESSAGE"

echo "Created tag: $TAG_NAME"
echo ""
echo "Tag message:"
echo "$TAG_MESSAGE"
