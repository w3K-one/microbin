
![Screenshot](.github/index.png)

# MicroBin

![Build](https://github.com/w3K-one/microbin/actions/workflows/rust.yml/badge.svg)
[![Docker Image](https://github.com/w3K-one/microbin/actions/workflows/BuildEmAll.yml/badge.svg)](https://hub.docker.com/r/w3kllc/microbin)
[![Docker Pulls](https://img.shields.io/docker/pulls/w3kllc/microbin?label=Docker%20pulls)](https://hub.docker.com/r/w3kllc/microbin)

MicroBin is a super tiny, feature-rich, configurable, self-contained and self-hosted paste bin web application. It is very easy to set up and use, and will only require a few megabytes of memory and disk storage. It takes only a couple minutes to set it up, why not give it a try now?

## Enhanced Fork

This is an enhanced fork of MicroBin with improved defaults and additional features:

- **Custom URLs** - Choose your own URL slug or use auto-generated ones
- **Real-time validation** - AJAX-powered availability checking as you type
- **QR code generation** - Enabled by default
- **Eternal pasta** - Never-expiring pastes by default
- **Hash IDs with animal names** - Shorter, friendlier URLs
- **CLI tool** - Full-featured command-line client with custom URL support
- **Telemetry disabled** - Privacy-focused defaults
- **Optimized Docker builds** - Multi-arch (amd64/arm64) with ~40min build times

### Try the Public Instance at [microbin.cc](https://microbin.cc)!

### Or host MicroBin yourself

Run with Docker ([DockerHub](https://hub.docker.com/r/w3kllc/microbin)):
```bash
docker run -d \
  -p 8080:8080 \
  -v microbin-data:/app/microbin_data \
  -e MICROBIN_PUBLIC_PATH=https://your-domain.com \
  w3kllc/microbin:latest
```

Or use Docker Compose:
```bash
curl -L -O https://raw.githubusercontent.com/w3K-one/microbin/master/compose.yaml
docker compose up -d
```

Or install from Cargo:
```bash
cargo install microbin
microbin
```

## Features

### Core Features
- Entirely self-contained executable, MicroBin is a single file!
- Server-side and client-side encryption
- File uploads (e.g. `microbin.cc/file/my-custom-url`)
- Raw text serving (e.g. `microbin.cc/raw/my-custom-url`)
- QR code generation (enabled by default in this fork)
- URL shortening and redirection
- Animal names OR hash IDs for upload identifiers
- SQLite and JSON database support
- Automatic dark mode and custom styling support

### Privacy Levels
- **Public** - Listed, anyone can view/edit/delete
- **Unlisted** - Not listed, URL required to access
- **Readonly** - Unlisted, password required to edit/delete
- **Private** - Server-side encrypted, password required to view
- **Secret** - Client-side encrypted, password required to view

### Upload Options
- Multiple expiration times: 1min, 10min, 1hour, 24hour, 3days, 1week, never
- Burn after read: Self-destruct after N reads
- Custom URLs: Choose your own memorable URL slug
- Password protection for all privacy levels
- Syntax highlighting for code snippets

## CLI Tool

This fork includes a full-featured command-line client for easy uploading:

```bash
# Upload text
echo "Hello world" | microbin

# Upload file with custom URL
microbin -f document.pdf -u my-document

# Upload with password protection
microbin -p private -P mypassword "Secret content"

# Upload with expiration and copy to clipboard
cat log.txt | microbin -e 1hour -c

# Upload with custom URL and return raw URL
microbin -u my-paste -r "Paste content"
```

### CLI Options
- `-f <file>` - Upload a file
- `-u <url>` - Custom URL slug (alphanumeric, hyphens, underscores)
- `-e <time>` - Expiration (1min, 10min, 1hour, 24hour, 3days, 1week, never)
- `-p <level>` - Privacy level (public, unlisted, readonly, private, secret)
- `-P <pass>` - Password for protected pastes
- `-b <num>` - Burn after N reads
- `-c` - Copy URL to clipboard
- `-r` - Return raw paste URL instead of view URL

Set `MICROBIN_URL` environment variable to use your own instance:
```bash
export MICROBIN_URL=https://your-instance.com
microbin "Your content"
```

## Custom URLs

One of the key enhancements in this fork is support for custom URLs:

- **Choose your own URL**: Instead of `microbin.cc/pasta/happy-dog-5`, use `microbin.cc/pasta/my-custom-url`
- **Real-time validation**: The web interface checks availability as you type
- **CLI support**: Use the `-u` flag to set custom URLs from the command line
- **Conflict detection**: Prevents duplicate custom URLs and conflicts with auto-generated slugs
- **Seamless fallback**: Leave blank to use auto-generated URLs

## What is a paste?

In MicroBin, a paste can be:

- Text that you want to share from one machine to another, e.g. some code
- A file that you want to share, e.g. a video, zip file, or image
- A URL redirection/shortener

## When is MicroBin useful?

You can use MicroBin:

- To send long texts to other people
- To send large files to other people
- To share secrets or sensitive documents securely with encryption
- As a URL shortener/redirect service
- To serve content on the web (config files, images, etc.) using Raw functionality
- To move files between your desktop and a server
- As a "postbox" service where people can upload but cannot see others' content
- To take quick notes with custom URLs for easy recall
- For temporary file sharing with burn-after-read
- For permanent storage with "never" expiration

...and many other things, why not get creative?

## Configuration

MicroBin is highly configurable through environment variables. Key settings:

```bash
MICROBIN_PORT=8080                    # Server port
MICROBIN_BIND=0.0.0.0                 # Bind address
MICROBIN_PUBLIC_PATH=https://...      # Public URL (required for QR codes)
MICROBIN_DATA_DIR=./microbin_data     # Data directory
MICROBIN_ADMIN_USERNAME=admin         # Admin username
MICROBIN_ADMIN_PASSWORD=m1cr0b1n      # Admin password
MICROBIN_QR=true                      # QR code generation (default in fork)
MICROBIN_ETERNAL_PASTA=true           # Allow never-expiring pastes (default in fork)
MICROBIN_HASH_IDS=true                # Use hash IDs instead of animal names
MICROBIN_DISABLE_TELEMETRY=true       # Disable telemetry (default in fork)
```

See the [.env file](https://github.com/w3K-one/microbin/blob/master/.env) for all options.

## Docker Deployment

Multi-architecture Docker images (amd64, arm64) are available at [w3kllc/microbin](https://hub.docker.com/r/w3kllc/microbin).

### Quick Start
```bash
docker run -d -p 8080:8080 w3kllc/microbin:latest
```

### With Reverse Proxy
See [DOCKER.md](DOCKER.md) for comprehensive examples with:
- Traefik
- Nginx Proxy Manager
- CloudPanel
- Caddy
- And more!

## Building from Source

```bash
# Clone the repository
git clone https://github.com/w3K-one/microbin.git
cd microbin

# Build release
cargo build --release

# Run
./target/release/microbin
```

## Admin Interface

Access the admin panel at `https://your-instance.com/@/` (default credentials: admin/m1cr0b1n)

Features:
- View all pastes
- Bulk delete operations
- Server statistics
- Configuration overview

## Links

- **Live Instance**: [microbin.cc](https://microbin.cc)
- **Docker Hub**: [w3kllc/microbin](https://hub.docker.com/r/w3kllc/microbin)
- **GitHub**: [w3K-one/microbin](https://github.com/w3K-one/microbin)
- **Original Project**: [szabodanika/microbin](https://github.com/szabodanika/microbin)

## License

MicroBin is available under the [BSD 3-Clause License](LICENSE).

Original work © Dániel Szabó 2022-2024
Fork enhancements © w3K 2024-2025
