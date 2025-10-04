#!/bin/bash
# Release Signing Script for Ectus-R
# Generates GPG signatures and SHA256 checksums for all release artifacts

set -euo pipefail

VERSION="${1:-}"
RELEASE_DIR="${2:-dist}"

if [ -z "$VERSION" ]; then
    echo "Error: Version required"
    echo "Usage: $0 <version> [release_dir]"
    exit 1
fi

echo "📝 Signing release artifacts for version $VERSION"

# Generate SHA256 checksums
echo "🔐 Generating SHA256 checksums..."
cd "$RELEASE_DIR"

find . -type f \( -name "*.tar.gz" -o -name "*.zip" -o -name "*.exe" -o -name "*.dmg" -o -name "*.AppImage" \) | while read file; do
    sha256sum "$file" >> SHA256SUMS.txt
    echo "  ✓ $(basename $file)"
done

# Sign checksums file with GPG (if GPG_KEY is available)
if [ -n "${GPG_PRIVATE_KEY:-}" ]; then
    echo "🔑 Signing checksums with GPG..."
    echo "$GPG_PRIVATE_KEY" | gpg --import --batch --yes 2>/dev/null || true
    gpg --batch --yes --detach-sign --armor SHA256SUMS.txt
    echo "  ✓ Created SHA256SUMS.txt.asc"
else
    echo "⚠️  GPG_PRIVATE_KEY not set - skipping GPG signature"
fi

# Generate verification script
cat > verify-checksums.sh << 'VERIFY_EOF'
#!/bin/bash
# Ectus-R Release Verification Script

set -e

echo "🔍 Verifying Ectus-R release checksums..."

if [ ! -f SHA256SUMS.txt ]; then
    echo "❌ SHA256SUMS.txt not found"
    exit 1
fi

# Verify GPG signature if available
if [ -f SHA256SUMS.txt.asc ]; then
    echo "🔑 Verifying GPG signature..."
    if command -v gpg &>/dev/null; then
        # Import Ectus-R public key
        gpg --keyserver keyserver.ubuntu.com --recv-keys 0xYOUR_KEY_ID 2>/dev/null || \
            echo "⚠️  Could not import GPG key automatically"

        if gpg --verify SHA256SUMS.txt.asc SHA256SUMS.txt 2>/dev/null; then
            echo "✅ GPG signature valid"
        else
            echo "⚠️  GPG signature verification failed"
        fi
    else
        echo "⚠️  gpg not installed - skipping signature verification"
    fi
fi

# Verify checksums
echo "🔐 Verifying file checksums..."
if sha256sum --ignore-missing -c SHA256SUMS.txt; then
    echo "✅ All checksums verified successfully"
else
    echo "❌ Checksum verification failed"
    exit 1
fi
VERIFY_EOF

chmod +x verify-checksums.sh

echo "✅ Release signing complete"
echo ""
echo "Files created:"
echo "  - SHA256SUMS.txt"
[ -f SHA256SUMS.txt.asc ] && echo "  - SHA256SUMS.txt.asc (GPG signature)"
echo "  - verify-checksums.sh"
echo ""
echo "Users can verify downloads with:"
echo "  curl -sSL https://github.com/Yatrogenesis/Ectus-R/releases/download/$VERSION/verify-checksums.sh | bash"
