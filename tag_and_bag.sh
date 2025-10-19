#!/usr/bin/env bash
set -e

# -----------------------------
# Usage check
# -----------------------------
if [ -z "$1" ]; then
  echo "Usage: $0 <new-version> (e.g., 1.2.3)"
  exit 1
fi

NEW_VERSION="$1"
echo "Updating Cargo.toml to version $NEW_VERSION"

# -----------------------------
# Update Cargo.toml
# -----------------------------
sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml

# -----------------------------
# Commit and tag
# -----------------------------
git add Cargo.toml
git commit -m "Bump version to $NEW_VERSION"
git tag "v$NEW_VERSION"

echo "Version updated and tagged as v$NEW_VERSION"
echo "Don't forget to push: git push && git push --tags"