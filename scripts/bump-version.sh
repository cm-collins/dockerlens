#!/usr/bin/env bash
set -euo pipefail

VERSION=${1:-}

if [[ -z "$VERSION" ]]; then
  echo "Usage: pnpm version:bump <version>"
  echo "Example: pnpm version:bump 0.1.0-alpha"
  exit 1
fi

# Validate semver (with optional pre-release label)
if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$ ]]; then
  echo "Error: '$VERSION' is not a valid semver (e.g. 1.0.0 or 0.1.0-alpha)"
  exit 1
fi

sed -i "s/\"version\": \"[^\"]*\"/\"version\": \"$VERSION\"/" package.json
sed -i "s/^version = \"[^\"]*\"/version = \"$VERSION\"/" src-tauri/Cargo.toml
sed -i "s/\"version\": \"[^\"]*\"/\"version\": \"$VERSION\"/" src-tauri/tauri.conf.json

echo "✓ Bumped to $VERSION"
echo ""
echo "Next steps:"
echo "  git add package.json src-tauri/Cargo.toml src-tauri/tauri.conf.json"
echo "  git commit -m \"chore: bump version to $VERSION\""
echo "  git tag v$VERSION"
echo "  git push origin main --tags"
