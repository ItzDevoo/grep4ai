#!/usr/bin/env bash
set -euo pipefail

# Release script for grepit
# Usage: ./scripts/release.sh 0.2.0

VERSION="${1:?Usage: ./scripts/release.sh <version>}"

echo "Releasing grepit v${VERSION}..."

# 1. Update Cargo.toml workspace version
sed -i "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml

# 2. Update all npm package.json versions
for dir in npm/grepit npm/grepit-win32-x64 npm/grepit-linux-x64 npm/grepit-linux-arm64 npm/grepit-darwin-x64 npm/grepit-darwin-arm64; do
  cd "$dir"
  npm version "$VERSION" --no-git-tag-version --allow-same-version
  cd - > /dev/null
done

# Update optionalDependencies in main package
node -e "
  const pkg = require('./npm/grepit/package.json');
  for (const dep of Object.keys(pkg.optionalDependencies || {})) {
    pkg.optionalDependencies[dep] = '${VERSION}';
  }
  require('fs').writeFileSync('./npm/grepit/package.json', JSON.stringify(pkg, null, 2) + '\n');
"

# 3. Run tests
echo "Running tests..."
cargo test --workspace

# 4. Commit and tag
git add -A
git commit -m "release: v${VERSION}"
git tag "v${VERSION}"

echo ""
echo "Done! To publish:"
echo "  git push origin main --tags"
echo ""
echo "This will trigger the GitHub Actions release workflow which:"
echo "  1. Builds binaries for all 5 platforms"
echo "  2. Creates a GitHub Release with downloadable archives"
echo "  3. Publishes all npm packages to the registry"
