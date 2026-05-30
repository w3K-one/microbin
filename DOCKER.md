# MicroBin — w3K Edition

A fast, self-hosted, privacy-first pastebin and file-sharing service written in Rust. Share text snippets, upload files, set optional passwords, custom URLs, and expiry times — entirely self-contained, no external dependencies, no tracking.

This is the **w3K fork** of [MicroBin](https://github.com/szabodanika/microbin), with production-hardened defaults and additional features. Live at **[microbin.cc](https://microbin.cc)**.

---

## What's Different in This Fork

| Feature | Upstream | w3K Fork |
|---|---|---|
| QR code generation | Disabled | **Enabled** |
| Eternal (never-expiring) pastes | Disabled | **Enabled** |
| Default expiry | 1 hour | **Never** |
| Telemetry | Enabled | **Disabled** |
| Hash IDs with animal names | Disabled | **Enabled** |
| Client-side encryption | Disabled | **Enabled** |
| Server-side encryption | Disabled | **Enabled** |
| Custom URL preservation | Broken | **Fixed** |
| HTML file inline rendering | No | **Yes** |
| Admin dashboard | Broken (`/admin` 404) | **Fixed** (`/@`) |

---

## Quick Start

```bash
docker run -d \
  -p 8080:8080 \
  -e MICROBIN_PUBLIC_PATH=https://paste.example.com \
  -v microbin-data:/app/microbin_data \
  --name microbin \
  w3kllc/microbin:latest
```

Visit `http://localhost:8080` — admin panel at `http://localhost:8080/@`.

---

## Supported Platforms

- `linux/amd64` — Intel/AMD 64-bit (desktops, servers, cloud VMs)
- `linux/arm64` — ARM 64-bit (Raspberry Pi 4+, Apple Silicon, AWS Graviton, Oracle Cloud)

Docker pulls the correct image for your platform automatically.

---

## Docker Compose

```yaml
services:
  microbin:
    image: w3kllc/microbin:latest
    container_name: microbin
    restart: unless-stopped
    ports:
      - "8080:8080"
    volumes:
      - microbin-data:/app/microbin_data
    environment:
      - MICROBIN_PORT=8080
      - MICROBIN_PUBLIC_PATH=https://paste.example.com
      - MICROBIN_ADMIN_USERNAME=admin
      - MICROBIN_ADMIN_PASSWORD=changeme
      - MICROBIN_QR=true
      - MICROBIN_ETERNAL_PASTA=true
      - MICROBIN_HASH_IDS=true
      - MICROBIN_DISABLE_TELEMETRY=true

volumes:
  microbin-data:
```

### With a reverse proxy (Traefik)

```yaml
services:
  microbin:
    image: w3kllc/microbin:latest
    container_name: microbin
    restart: unless-stopped
    volumes:
      - microbin-data:/app/microbin_data
    environment:
      - MICROBIN_PUBLIC_PATH=https://paste.example.com
      - MICROBIN_ADMIN_USERNAME=admin
      - MICROBIN_ADMIN_PASSWORD=changeme
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.microbin.rule=Host(`paste.example.com`)"
      - "traefik.http.routers.microbin.entrypoints=websecure"
      - "traefik.http.routers.microbin.tls.certresolver=letsencrypt"
      - "traefik.http.services.microbin.loadbalancer.server.port=8080"

volumes:
  microbin-data:
```

### With Nginx Proxy Manager / CloudPanel

Point your domain to the container's port 8080. Set `MICROBIN_PUBLIC_PATH` to your full public URL including `https://`.

---

## Configuration

| Variable | Default | Description |
|---|---|---|
| `MICROBIN_PORT` | `8080` | Port to listen on |
| `MICROBIN_BIND` | `0.0.0.0` | Bind address |
| `MICROBIN_PUBLIC_PATH` | *(unset)* | Full public URL — required for QR codes and copy-URL |
| `MICROBIN_DATA_DIR` | `microbin_data` | Data directory path |
| `MICROBIN_ADMIN_USERNAME` | `admin` | Admin panel username |
| `MICROBIN_ADMIN_PASSWORD` | `m1cr0b1n` | Admin panel password — **change this** |
| `MICROBIN_QR` | `true`* | Enable QR code generation |
| `MICROBIN_ETERNAL_PASTA` | `true`* | Allow never-expiring pastes |
| `MICROBIN_HASH_IDS` | `true`* | Use hash IDs with animal names |
| `MICROBIN_PRIVATE` | `true` | Enable private pastes |
| `MICROBIN_EDITABLE` | `true` | Allow paste editing |
| `MICROBIN_ENCRYPTION_CLIENT_SIDE` | `true` | Enable client-side encryption |
| `MICROBIN_ENCRYPTION_SERVER_SIDE` | `true` | Enable server-side encryption |
| `MICROBIN_ENABLE_BURN_AFTER` | `true` | Enable burn-after-read |
| `MICROBIN_HIGHLIGHTSYNTAX` | `true` | Enable syntax highlighting |
| `MICROBIN_DISABLE_TELEMETRY` | `true`* | Disable telemetry |
| `MICROBIN_READONLY` | `false` | Require upload password |
| `MICROBIN_PURE_HTML` | `false` | Disable CSS/JS (accessibility mode) |
| `MICROBIN_LIST_SERVER` | `false` | Enable public paste listing |
| `MICROBIN_HIDE_LOGO` | `false` | Hide the MicroBin logo |
| `MICROBIN_HIDE_FOOTER` | `false` | Hide footer |

*Defaults differ from upstream — set by this fork.

### Privacy Levels

| Level | Listed | URL Required | Password to View | Encrypted |
|---|---|---|---|---|
| `public` | Yes | No | No | No |
| `unlisted` | No | Yes | No | No |
| `readonly` | No | Yes | No | No |
| `private` | No | Yes | Yes | Server-side AES-256 |
| `secret` | No | Yes | Yes | Client-side (browser) |

### Expiry Options

`1min` · `10min` · `1hour` · `24hour` · `3days` · `1week` · `never`

---

## Admin Panel

Access at `<your-url>/@` — sign in with your configured username and password.

The admin panel lists all pastes regardless of privacy level, allows deletion, and shows server status and version info.

---

## HTML File Rendering

Upload any `.html` or `.htm` file and it renders inline in the browser instead of downloading. A small non-printable navigation button (bottom-right) links back to MicroBin. The button is hidden automatically when printing or saving as PDF.

This makes MicroBin useful for sharing formatted documents — resumes, reports, proposals — with or without a password, at a clean custom URL.

```
microbin.cc/juanitaresume          # public — visit and read
microbin.cc/q4report               # unlisted — share link to read
microbin.cc/contractdraft          # private — password required
```

---

## Custom URLs

Set a custom URL slug when creating a paste. It persists across all operations including auth redirects, incorrect-password flows, edit, and remove.

URLs must be alphanumeric with hyphens/underscores, max 100 characters.

---

## Image Tags

| Tag | Description |
|---|---|
| `latest` | Most recent stable release |
| `v2.x.x` | Specific version |
| `buildcache-linux-amd64` | Build layer cache (internal) |
| `buildcache-linux-arm64` | Build layer cache (internal) |

---

## Building Locally

```bash
# Single platform
docker build -t microbin:local .

# Multi-platform (requires buildx)
docker buildx build --platform linux/amd64,linux/arm64 -t microbin:local .

# From source (Rust)
cargo build --release
./target/release/microbin --port 8080
```

---

## Links

- **Live instance:** [microbin.cc](https://microbin.cc)
- **Fork repository:** [github.com/w3K-one/microbin](https://github.com/w3K-one/microbin)
- **Docker Hub:** [hub.docker.com/r/w3kllc/microbin](https://hub.docker.com/r/w3kllc/microbin)
- **Upstream project:** [github.com/szabodanika/microbin](https://github.com/szabodanika/microbin)

---

## License

BSD-3-Clause — see [LICENSE](https://github.com/w3K-one/microbin/blob/master/LICENSE).
