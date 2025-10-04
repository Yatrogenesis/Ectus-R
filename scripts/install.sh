#!/bin/bash
# Universal Ectus-R Installer
# Supports: Linux (x64), macOS (x64, ARM64), Windows (via WSL/Git Bash)
# Usage: curl -sSL https://install.ectus-r.com | sh

set -euo pipefail

# Configuration
REPO="Yatrogenesis/Ectus-R"
INSTALL_DIR="${ECTUS_INSTALL_DIR:-$HOME/.ectus-r}"
BIN_DIR="${ECTUS_BIN_DIR:-$HOME/.local/bin}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
info() { echo -e "${BLUE}ℹ${NC} $*"; }
success() { echo -e "${GREEN}✓${NC} $*"; }
warning() { echo -e "${YELLOW}⚠${NC} $*"; }
error() { echo -e "${RED}✗${NC} $*"; exit 1; }

# Detect platform
detect_platform() {
    local os arch

    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
        Linux*)
            case "$arch" in
                x86_64) echo "linux-x64" ;;
                aarch64|arm64) echo "linux-arm64" ;;
                *) error "Unsupported architecture: $arch" ;;
            esac
            ;;
        Darwin*)
            case "$arch" in
                x86_64) echo "macos-x64" ;;
                arm64) echo "macos-arm64" ;;
                *) error "Unsupported architecture: $arch" ;;
            esac
            ;;
        MINGW*|MSYS*|CYGWIN*)
            echo "windows-x64"
            ;;
        *)
            error "Unsupported OS: $os"
            ;;
    esac
}

# Get latest release version
get_latest_version() {
    local version
    version=$(curl -sSL "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | cut -d'"' -f4)

    if [ -z "$version" ]; then
        error "Could not determine latest version"
    fi

    echo "$version"
}

# Download and install
install_ectus() {
    local platform version download_url archive_name

    info "Detecting platform..."
    platform=$(detect_platform)
    success "Platform: $platform"

    info "Fetching latest version..."
    version=$(get_latest_version)
    success "Latest version: $version"

    # Determine file extension
    case "$platform" in
        linux-*) archive_name="ectus-r-${platform}.tar.gz" ;;
        macos-*) archive_name="ectus-r-${platform}.tar.gz" ;;
        windows-*) archive_name="ectus-r-${platform}-portable.zip" ;;
    esac

    download_url="https://github.com/$REPO/releases/download/$version/$archive_name"

    info "Creating installation directory..."
    mkdir -p "$INSTALL_DIR"
    mkdir -p "$BIN_DIR"

    info "Downloading Ectus-R $version..."
    local temp_file="/tmp/$archive_name"

    if command -v wget &>/dev/null; then
        wget -q --show-progress -O "$temp_file" "$download_url" || error "Download failed"
    elif command -v curl &>/dev/null; then
        curl -sSL -o "$temp_file" --progress-bar "$download_url" || error "Download failed"
    else
        error "Neither wget nor curl found - cannot download"
    fi

    success "Downloaded"

    info "Extracting archive..."
    case "$archive_name" in
        *.tar.gz)
            tar -xzf "$temp_file" -C "$INSTALL_DIR" --strip-components=1
            ;;
        *.zip)
            if command -v unzip &>/dev/null; then
                unzip -q "$temp_file" -d "$INSTALL_DIR"
            else
                error "unzip not found - cannot extract"
            fi
            ;;
    esac

    rm "$temp_file"
    success "Extracted to $INSTALL_DIR"

    info "Creating symlinks..."
    ln -sf "$INSTALL_DIR/bin/ectus-cli" "$BIN_DIR/ectus-cli" 2>/dev/null || true
    ln -sf "$INSTALL_DIR/bin/aion-web-api" "$BIN_DIR/aion-web-api" 2>/dev/null || true

    success "Installed binaries to $BIN_DIR"

    # Add to PATH if not present
    local shell_rc=""
    if [ -n "${BASH_VERSION:-}" ]; then
        shell_rc="$HOME/.bashrc"
    elif [ -n "${ZSH_VERSION:-}" ]; then
        shell_rc="$HOME/.zshrc"
    fi

    if [ -n "$shell_rc" ] && [ -f "$shell_rc" ]; then
        if ! grep -q "$BIN_DIR" "$shell_rc"; then
            info "Adding $BIN_DIR to PATH in $shell_rc"
            echo "" >> "$shell_rc"
            echo "# Ectus-R" >> "$shell_rc"
            echo "export PATH=\"\$PATH:$BIN_DIR\"" >> "$shell_rc"
            warning "Please run: source $shell_rc"
        fi
    fi

    echo ""
    success "Ectus-R $version installed successfully!"
    echo ""
    info "Quick start:"
    echo "  ectus-cli --version"
    echo "  ectus-cli new"
    echo ""
    info "Documentation: https://ectus-r.com/docs"
    info "Support: https://github.com/$REPO/issues"
}

# Verify installation
verify_installation() {
    info "Verifying installation..."

    if [ -x "$INSTALL_DIR/bin/ectus-cli" ]; then
        local version
        version=$("$INSTALL_DIR/bin/ectus-cli" --version 2>/dev/null || echo "unknown")
        success "ectus-cli installed: $version"
    else
        error "Installation verification failed"
    fi
}

# Main
main() {
    echo ""
    echo "╔═══════════════════════════════════════╗"
    echo "║   Ectus-R Universal Installer         ║"
    echo "║   Autonomous Software Engineering     ║"
    echo "╚═══════════════════════════════════════╝"
    echo ""

    install_ectus
    verify_installation
}

main "$@"
