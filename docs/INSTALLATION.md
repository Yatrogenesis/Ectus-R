# Ectus-R Installation Guide

Complete installation instructions for all supported platforms.

## Quick Install (Recommended)

### Linux & macOS

```bash
curl -sSL https://install.ectus-r.com | sh
```

Or with wget:

```bash
wget -qO- https://install.ectus-r.com | sh
```

### Windows

**Option 1: Download Installer (Recommended)**

1. Download the latest `.exe` installer from [Releases](https://github.com/Yatrogenesis/Ectus-R/releases)
2. Run the installer
3. Follow the setup wizard
4. Ectus-R will be available in Start Menu and `PATH`

**Option 2: Portable Version**

1. Download `ectus-r-windows-x64-portable.zip`
2. Extract to desired location (e.g., `C:\Program Files\Ectus-R`)
3. Add to PATH manually or run from extraction directory

**Option 3: Git Bash / WSL**

```bash
curl -sSL https://install.ectus-r.com | sh
```

---

## Platform-Specific Installation

### Linux

#### Debian/Ubuntu (AppImage)

```bash
# Download AppImage
wget https://github.com/Yatrogenesis/Ectus-R/releases/latest/download/ectus-r-linux-x64.AppImage

# Make executable
chmod +x ectus-r-linux-x64.AppImage

# Run
./ectus-r-linux-x64.AppImage

# Optional: Install to system
sudo mv ectus-r-linux-x64.AppImage /usr/local/bin/ectus-r
```

#### Arch Linux (AUR)

```bash
# Using yay
yay -S ectus-r

# Or manually
git clone https://aur.archlinux.org/ectus-r.git
cd ectus-r
makepkg -si
```

#### Universal (tar.gz)

```bash
# Download
wget https://github.com/Yatrogenesis/Ectus-R/releases/latest/download/ectus-r-linux-x64.tar.gz

# Extract
tar -xzf ectus-r-linux-x64.tar.gz -C ~/.local/share/ectus-r

# Add to PATH
echo 'export PATH="$PATH:$HOME/.local/share/ectus-r/bin"' >> ~/.bashrc
source ~/.bashrc
```

### macOS

#### Homebrew (Recommended)

```bash
brew tap yatrogenesis/ectus-r
brew install ectus-r
```

#### DMG Installer

1. Download `ectus-r-macos-{x64,arm64}.dmg`
2. Open the DMG file
3. Drag Ectus-R to Applications folder
4. Launch from Applications or Spotlight

#### Manual (tar.gz)

```bash
# Intel Macs
wget https://github.com/Yatrogenesis/Ectus-R/releases/latest/download/ectus-r-macos-x64.tar.gz

# Apple Silicon Macs
wget https://github.com/Yatrogenesis/Ectus-R/releases/latest/download/ectus-r-macos-arm64.tar.gz

# Extract
tar -xzf ectus-r-macos-*.tar.gz -C /usr/local

# Verify installation
ectus-cli --version
```

### Windows

#### Chocolatey

```powershell
choco install ectus-r
```

#### Scoop

```powershell
scoop bucket add yatrogenesis https://github.com/Yatrogenesis/scoop-bucket
scoop install ectus-r
```

#### WinGet

```powershell
winget install Yatrogenesis.EctusR
```

---

## Docker Installation

### Minimal Setup (Development)

```bash
# Clone repository
git clone https://github.com/Yatrogenesis/Ectus-R.git
cd Ectus-R

# Start only essential services
docker-compose -f docker-compose.minimal.yml up -d

# Configure LLM API keys
export GROQ_API_KEY="your_groq_key"
export OPENAI_API_KEY="your_openai_key"

# Build and run
cargo build --release
cargo run --bin aion-web-api
```

Access: `http://localhost:8080`

### Full Stack (Production)

```bash
# Clone repository
git clone https://github.com/Yatrogenesis/Ectus-R.git
cd Ectus-R

# Configure environment
cp .env.example .env
# Edit .env with your API keys and configuration

# Start all services
docker-compose up -d

# Verify services
docker-compose ps
```

Services:
- Web API: `http://localhost:8080`
- Prometheus: `http://localhost:9090`
- Grafana: `http://localhost:3000` (admin/admin)
- Jaeger: `http://localhost:16686`

---

## Building from Source

### Prerequisites

- **Rust** 1.70+ (`rustup`)
- **PostgreSQL** 13+ (or Docker)
- **Node.js** 18+ (for web dashboard)
- **Git**

### Build Steps

```bash
# 1. Clone repository
git clone https://github.com/Yatrogenesis/Ectus-R.git
cd Ectus-R

# 2. Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable

# 3. Build release binary
cargo build --release

# 4. Install to system (optional)
sudo cp target/release/ectus-cli /usr/local/bin/
sudo cp target/release/aion-web-api /usr/local/bin/

# 5. Verify installation
ectus-cli --version
```

### Development Build

```bash
# Debug build (faster compilation)
cargo build

# Run tests
cargo test --all

# Run with hot reload
cargo watch -x run
```

---

## Verification

### Verify Download Integrity

```bash
# Download checksums
wget https://github.com/Yatrogenesis/Ectus-R/releases/latest/download/SHA256SUMS.txt

# Download GPG signature (optional)
wget https://github.com/Yatrogenesis/Ectus-R/releases/latest/download/SHA256SUMS.txt.asc

# Verify checksums
sha256sum -c SHA256SUMS.txt --ignore-missing

# Verify GPG signature (if GPG installed)
gpg --keyserver keyserver.ubuntu.com --recv-keys 0xYOUR_KEY_ID
gpg --verify SHA256SUMS.txt.asc SHA256SUMS.txt
```

### Verify Installation

```bash
# Check version
ectus-cli --version

# Run health check
ectus-cli health

# Create test project
ectus-cli new --name test-project
```

---

## Configuration

### LLM API Keys (Required)

Ectus-R requires at least one LLM provider API key:

```bash
# Groq (Recommended - Free tier available)
export GROQ_API_KEY="gsk_..."

# OpenAI (High quality)
export OPENAI_API_KEY="sk-..."

# GitHub Models (Free tier)
export GITHUB_TOKEN="ghp_..."

# HuggingFace (Open models)
export HUGGINGFACE_API_KEY="hf_..."

# Cloudflare AI (Edge inference)
export CLOUDFLARE_API_TOKEN="..."
export CLOUDFLARE_ACCOUNT_ID="..."
```

### Environment Variables

```bash
# Database
export DATABASE_URL="postgresql://user:password@localhost:5432/ectus_r"

# Redis (optional)
export REDIS_URL="redis://localhost:6379"

# Monitoring
export PROMETHEUS_PORT="9091"
export JAEGER_ENDPOINT="http://localhost:4317"

# Web API
export WEB_API_PORT="8080"
export WEB_API_HOST="0.0.0.0"

# Logging
export RUST_LOG="info,aion_core=debug"
```

### Configuration File

Create `~/.config/ectus-r/config.toml`:

```toml
[llm]
default_provider = "groq"
fallback_enabled = true

[generation]
max_retries = 3
timeout_seconds = 300

[monitoring]
prometheus_enabled = true
jaeger_enabled = true
metrics_port = 9091

[database]
connection_pool_size = 10
max_connections = 100

[security]
enable_rate_limiting = true
max_requests_per_minute = 60
```

---

## Troubleshooting

### Common Issues

#### "Command not found: ectus-cli"

**Solution:** Add to PATH

```bash
# Bash
echo 'export PATH="$PATH:$HOME/.local/bin"' >> ~/.bashrc
source ~/.bashrc

# Zsh
echo 'export PATH="$PATH:$HOME/.local/bin"' >> ~/.zshrc
source ~/.zshrc
```

#### "No LLM provider configured"

**Solution:** Set at least one API key

```bash
export GROQ_API_KEY="your_key_here"
# Or add to ~/.bashrc for persistence
```

#### "Database connection failed"

**Solution:** Start PostgreSQL

```bash
# Docker
docker-compose up -d postgres

# Or install locally
# Ubuntu/Debian
sudo apt install postgresql
sudo systemctl start postgresql

# macOS
brew install postgresql
brew services start postgresql
```

#### "Port 8080 already in use"

**Solution:** Change port

```bash
export WEB_API_PORT="8081"
cargo run --bin aion-web-api
```

---

## Uninstallation

### Linux/macOS

```bash
# Remove binaries
rm -f ~/.local/bin/ectus-cli
rm -f ~/.local/bin/aion-web-api

# Remove installation directory
rm -rf ~/.local/share/ectus-r

# Remove configuration
rm -rf ~/.config/ectus-r

# Remove from PATH (edit ~/.bashrc or ~/.zshrc)
# Delete the line: export PATH="$PATH:$HOME/.local/bin"
```

### Windows

**Installer version:**
1. Open "Add or Remove Programs"
2. Find "Ectus-R"
3. Click Uninstall

**Portable version:**
1. Delete extraction folder
2. Remove from PATH environment variable

---

## Next Steps

After installation:

1. **Quick Start Tutorial:** [docs/QUICKSTART.md](QUICKSTART.md)
2. **CLI Reference:** [docs/CLI.md](CLI.md)
3. **Architecture Guide:** [ARCHITECTURE.md](../ARCHITECTURE.md)
4. **Examples:** [examples/](../examples/)

---

## Support

- **Documentation:** https://ectus-r.com/docs
- **GitHub Issues:** https://github.com/Yatrogenesis/Ectus-R/issues
- **Discord Community:** https://discord.gg/ectus-r
- **Email Support:** support@ectus-r.com

---

© 2025 Ectus-R Project. Installation verified for Linux, macOS, and Windows.
