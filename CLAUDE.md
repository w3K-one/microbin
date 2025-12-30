# CLAUDE.md - MicroBin Fork Project Guide

## Project Overview

This is Andrew's fork of MicroBin, a Rust-based pastebin and URL shortener service. The fork is maintained at **https://github.com/w3K-one/microbin** with Docker images published to **w3kllc/microbin** on Docker Hub.

**Live Instance:** https://microbin.cc

### What Makes This Fork Different

This fork differentiates itself through enhanced default configurations:
- **QR code generation** enabled by default
- **Eternal pasta** functionality (never-expiring pastes)
- **Hash IDs with animal names** for shorter, friendlier URLs
- **"Never" as default expiry** setting
- **Telemetry disabled** for privacy
- Enhanced defaults implemented in source code, not just environment variables

---

## Repository Structure

```
microbin/
├── src/
│   ├── main.rs                    # Application entry point
│   ├── args.rs                    # CLI arguments and config
│   ├── pasta.rs                   # Pasta (paste) data structures
│   ├── endpoints/                 # HTTP route handlers
│   │   ├── admin.rs               # Admin interface
│   │   ├── auth_admin.rs          # Admin authentication
│   │   ├── auth_upload.rs         # Upload authentication
│   │   ├── create.rs              # Paste creation
│   │   ├── edit.rs                # Paste editing
│   │   ├── errors.rs              # Error handlers
│   │   ├── file.rs                # File downloads
│   │   ├── guide.rs               # User guide
│   │   ├── list.rs                # Paste listing
│   │   ├── pasta.rs               # Paste viewing
│   │   ├── qr.rs                  # QR code generation
│   │   ├── remove.rs              # Paste deletion
│   │   └── static_resources.rs    # Static file serving
│   └── util/
│       ├── animalnumbers.rs       # Animal name ID generation
│       ├── auth.rs                # Authentication utilities
│       ├── db.rs                  # Database abstraction
│       ├── db_json.rs             # JSON database implementation
│       ├── db_sqlite.rs           # SQLite database implementation
│       ├── hashids.rs             # Hash ID generation
│       ├── http_client.rs         # HTTP client utilities
│       ├── misc.rs                # Miscellaneous utilities
│       ├── syntaxhighlighter.rs   # Syntax highlighting
│       ├── telemetry.rs           # Telemetry handling
│       └── version.rs             # Version management
├── templates/                     # Askama HTML templates
│   ├── index.html                 # Main paste creation form
│   ├── upload.html                # Paste view page
│   ├── edit.html                  # Paste editing page
│   ├── admin.html                 # Admin dashboard
│   ├── guide.html                 # User guide
│   ├── header.html                # Common header
│   ├── footer.html                # Common footer
│   └── assets/                    # Static assets (JS, CSS)
├── .github/workflows/
│   ├── BuildEmAll.yml             # Multi-arch Docker build workflow
│   ├── release.yml                # GitHub release workflow
│   ├── rust.yml                   # Rust build/test workflow
│   └── rust-clippy.yml            # Clippy linting workflow
├── Cargo.toml                     # Rust package configuration
├── Cargo.lock                     # Dependency lock file
├── Dockerfile                     # Multi-arch Docker build
├── compose.yaml                   # Docker Compose example
├── docker-setup.sh                # Quick Docker setup script
├── .env                           # Environment configuration
├── render.yaml                    # Render.com deployment config
└── SECURITY.md                    # Security policy
```

---

## Technology Stack

### Core Application
- **Language:** Rust (edition 2021, MSRV 1.74.0)
- **Web Framework:** Actix-web 4.x
- **Template Engine:** Askama (compile-time templates)
- **Database:** SQLite (default) or JSON file storage
- **Encryption:** AES-256 via magic-crypt

### Key Dependencies
```toml
actix-web = "4"
actix-multipart = "0.7.2"
actix-web-httpauth = "0.8.2"
askama = "0.10"
rusqlite = "0.32" (feature-gated)
magic-crypt = "3.1.13"
qrcode-generator = "4.1.9"
syntect = "5.2.0"
reqwest = "0.12"
```

### Build Features
- `default` - Standard build with SQLite, OpenSSL TLS, zstd compression
- `no-c-deps` - Pure Rust build with rustcrypto TLS, no C dependencies

### Docker & CI/CD
- **Multi-architecture:** linux/amd64, linux/arm64
- **Base Image:** debian:bookworm-slim (production)
- **Build Image:** rust:latest (build stage)
- **Registry:** Docker Hub (w3kllc/microbin)

---

## Build Commands

### Local Development
```bash
# Build debug
cargo build

# Build release
cargo build --release

# Run locally
cargo run

# Run with specific args
cargo run -- --port 8080 --bind 0.0.0.0

# Run tests
cargo test

# Run clippy
cargo clippy --all-features
```

### Docker Commands
```bash
# Build multi-arch locally
docker buildx build --platform linux/amd64,linux/arm64 -t w3kllc/microbin:latest .

# Run container
docker run -d -p 8080:8080 -v microbin-data:/app/microbin_data w3kllc/microbin:latest

# Docker Compose
docker compose up -d
```

---

## Configuration Reference

### Critical Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `MICROBIN_PORT` | 8080 | Server port |
| `MICROBIN_BIND` | 0.0.0.0 | Bind address |
| `MICROBIN_PUBLIC_PATH` | unset | Public URL (required for QR/links) |
| `MICROBIN_DATA_DIR` | microbin_data | Data directory |
| `MICROBIN_ADMIN_USERNAME` | admin | Admin username |
| `MICROBIN_ADMIN_PASSWORD` | m1cr0b1n | Admin password |

### Feature Toggles

| Variable | Default | Description |
|----------|---------|-------------|
| `MICROBIN_QR` | false | Enable QR codes |
| `MICROBIN_ETERNAL_PASTA` | false | Allow never-expiring pastes |
| `MICROBIN_HASH_IDS` | false | Use hash IDs instead of animal names |
| `MICROBIN_PRIVATE` | true | Enable private pastes |
| `MICROBIN_EDITABLE` | true | Allow paste editing |
| `MICROBIN_ENABLE_BURN_AFTER` | true | Enable burn after read |
| `MICROBIN_ENCRYPTION_CLIENT_SIDE` | true | Client-side encryption |
| `MICROBIN_ENCRYPTION_SERVER_SIDE` | true | Server-side encryption |
| `MICROBIN_HIGHLIGHTSYNTAX` | true | Syntax highlighting |
| `MICROBIN_DISABLE_TELEMETRY` | false | Disable telemetry |

### Expiration Options
- `1min`, `10min`, `1hour`, `24hour`, `3days`, `1week`, `never`

### Privacy Levels
- `public` - Listed, anyone can view/edit/delete
- `unlisted` - Not listed, URL required to access
- `readonly` - Unlisted, password to edit/delete
- `private` - Server-side encrypted, password to view
- `secret` - Client-side encrypted, password to view

---

## GitHub Actions Workflow (BuildEmAll.yml)

The "Build-Em-All" workflow handles multi-architecture Docker builds:

### Trigger Conditions
- Push to `main` or `master` branch
- Manual workflow dispatch

### Version Tagging Strategy
1. Extract version from commit message (e.g., `v1.2.3 Fix bug`)
2. If no version, increment latest git tag patch version
3. If no tags, use date-based version (`v25.01.15-1430`)

### Workflow Steps
1. Checkout with full history (`fetch-depth: 0`)
2. Determine version tag
3. Extract Docker metadata
4. Setup QEMU (multi-arch emulation)
5. Setup Docker Buildx
6. Login to Docker Hub
7. Build and push multi-platform image
8. Create GitHub Release
9. Update Docker Hub description

### Required Secrets
- `DOCKERHUB_USERNAME`
- `DOCKERHUB_TOKEN`

---

## Work Completed

### 1. Multi-Architecture Docker Builds
- Reduced build times from ~3 hours to ~40 minutes
- Target architectures: amd64, arm64 (covers 99% of use cases)
- Uses `debian:bookworm-slim` base image (not bitnami/minideb)
- Native builds via QEMU emulation

### 2. Enhanced Default Configuration
Implemented directly in source/config:
- QR code generation enabled
- Eternal pasta enabled
- Hash IDs with animal names
- "Never" as default expiry
- Telemetry disabled
- Client and server-side encryption enabled

### 3. CLI Tool (`microbin` script)
A comprehensive bash CLI for microbin.cc:

**Upload Features:**
- Text content upload (argument or stdin pipe)
- File uploads (`-f <file>`)
- Privacy levels (`-p public|unlisted|readonly|private|secret`)
- Expiration settings (`-e 1min|10min|1hour|24hour|3days|1week|never`)
- Password protection (`-P <password>`)
- Burn after read (`-b <count>`)
- Raw URL output (`-r`)
- Clipboard copy (`-c`)

**Retrieval Features:**
- Get paste by ID (`-g <paste-id>`)
- Auto-detect content type (text/file/URL)
- Save to file (`-o <filename>`)
- Password support for protected pastes

### 4. Documentation
- Docker Compose examples for multiple reverse proxy setups:
  - Traefik
  - Nginx Proxy Manager
  - CloudPanel
- Comprehensive DOCKER.md with 8+ deployment examples
- CLI usage documentation with 30+ examples

### 5. WordPress Plugin Conversion Plan
Detailed conversion documentation created:
- Plugin architecture design
- Database schema (pastes, files, URLs tables)
- REST API endpoints
- Admin interface mockups
- Core PHP classes:
  - `class-microbin-activator.php`
  - `class-microbin-paste-handler.php`
  - `class-microbin-rest-api.php`

---

## Areas for Future Development

### Planned / In Progress
1. **WordPress Plugin Implementation** - Convert MicroBin to WordPress plugin
2. **Enhanced Reverse Proxy Docs** - More examples and troubleshooting
3. **CLI Tool Improvements** - Binary detection, better error handling
4. **Source Code Defaults** - Move more defaults from env vars to code

### Potential Improvements
- API documentation (OpenAPI/Swagger)
- Rate limiting configuration
- Custom themes/CSS support improvements
- Webhooks for paste events
- S3/external storage support

---

## Development Patterns & Conventions

### Code Style
- Follow Rust idioms and clippy recommendations
- Use meaningful variable/function names
- Document public APIs with doc comments
- Handle errors with proper Result types

### Commit Messages
- Version releases: Start with `v1.2.3` to trigger auto-tagging
- Feature: `feat: Add feature description`
- Fix: `fix: Fix issue description`
- Docs: `docs: Update documentation`

### Branch Strategy
- `master` or `main` - Production branch, triggers Docker builds
- Feature branches for development

### Naming Conventions
- Workflow names: "Build-Em-All" style
- Environment variables: `MICROBIN_` prefix, SCREAMING_SNAKE_CASE
- Rust files: snake_case
- Docker image: `w3kllc/microbin`

### Testing Approach
- `cargo test` for unit tests
- Manual testing for endpoint changes
- Docker build verification for releases

---

## Agent Instructions

### Quick Start for New Tasks

1. **Always check project knowledge first** - Search for relevant context before making changes
2. **Reference the correct fork** - Use `w3kllc/microbin` not `danielszabo99/microbin`
3. **Test Docker builds** - Verify multi-arch compatibility for infrastructure changes
4. **Maintain enhanced defaults** - Preserve the fork's differentiated configuration

### Common Task Patterns

#### Modifying Rust Code
```bash
# 1. Make changes to src/ files
# 2. Build and test
cargo build
cargo test
cargo clippy

# 3. If changing templates, verify rendering
cargo run

# 4. Test Docker build
docker build -t test-build .
```

#### Updating GitHub Workflow
```bash
# 1. Edit .github/workflows/BuildEmAll.yml
# 2. Validate YAML syntax
# 3. Test workflow locally with act (optional)
act push

# 4. Push to trigger workflow
git push
```

#### Updating Docker Configuration
```bash
# 1. Edit Dockerfile or compose.yaml
# 2. Test local build
docker build -t test .
docker run --rm -p 8080:8080 test

# 3. Test multi-arch build
docker buildx build --platform linux/amd64,linux/arm64 -t test .
```

### Key Files to Review First
1. `Cargo.toml` - Dependencies and features
2. `src/args.rs` - Configuration options
3. `src/main.rs` - Application structure
4. `.github/workflows/BuildEmAll.yml` - CI/CD pipeline
5. `Dockerfile` - Container build process

### Important Reminders
- **Architecture coverage:** amd64 and arm64 cover 99% of deployments
- **Base image:** Always use `debian:bookworm-slim` for runtime
- **Secrets:** Never commit credentials; use GitHub secrets
- **Fork references:** Always point to `w3kllc` org, not original
- **Defaults:** Prefer source code defaults over environment variables

---

## Quick Reference Commands

```bash
# Build
cargo build --release

# Run locally
cargo run -- --port 8080

# Test
cargo test

# Lint
cargo clippy --all-features

# Docker build (local)
docker build -t w3kllc/microbin:test .

# Docker multi-arch build
docker buildx build --platform linux/amd64,linux/arm64 \
  -t w3kllc/microbin:latest --push .

# Run container
docker run -d -p 8080:8080 \
  -e MICROBIN_PUBLIC_PATH=https://microbin.cc \
  -v microbin-data:/app/microbin_data \
  w3kllc/microbin:latest
```

---

## Links & Resources

- **Fork Repository:** https://github.com/w3K-one/microbin
- **Docker Hub:** https://hub.docker.com/r/w3kllc/microbin
- **Live Instance:** https://microbin.cc
- **Original Project:** https://github.com/szabodanika/microbin
- **Original Docs:** https://microbin.eu/docs/intro

---

## Contact & Maintenance

This fork is maintained by Andrew (w3K). For issues specific to this fork, use the GitHub issues on the w3K-one/microbin repository.
