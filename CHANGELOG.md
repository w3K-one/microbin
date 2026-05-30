# Changelog

All notable changes to the w3K MicroBin fork are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## [2.1.1] - 2026-05-29

### Fixed

- **Custom URLs not sticking** — all redirect paths (auth, incorrect password, edit, remove) were using the generated animal/hash ID instead of the original URL slug. Custom URLs now persist across every flow including encrypted paste auth, wrong-password bounce, edit, and delete. Root cause: 15 redirect sites across `pasta.rs`, `file.rs`, `edit.rs`, and `remove.rs` were calling `id_as_animals()` instead of the `slug` variable already captured from the URL path.

- **Admin panel 404** — `auth_admin.html` form `action` was pointing to `/admin/` but the route handler lives at `POST /@`. Navigating to `/@` now correctly authenticates and shows the admin dashboard.

### Added

- **`Pasta::slug()` method** — returns `custom_url` if set, otherwise falls back to `id_as_animals()`. All URL construction now goes through this single method, including template links in `upload.html`.

- **`PastaFile::is_html()` method** — detects `.html` and `.htm` extensions. Used by the file server and pasta view router.

- **HTML file inline rendering** — uploading a `.html` or `.htm` file now serves it rendered in the browser instead of as a download attachment. Works for both unencrypted (`GET /file/{slug}`) and server-side encrypted (`POST /secure_file/{slug}`) files. Visiting the paste URL for a non-encrypted HTML file redirects directly to the inline view — no intermediate download step.

- **Non-printable navigation button** — injected before `</body>` in all HTML file responses. Fixed-position dark circle (bottom-right), circular-arrow SVG icon, links back to `/`. Hidden via `@media print { display: none !important }` so it never appears in PDF exports or paper prints.

### Build

- **Switched to native ARM builds** — `BuildEmAll.yml` now runs `linux/amd64` on `ubuntu-latest` and `linux/arm64` on `ubuntu-24.04-arm` (GitHub-hosted native ARM64 runner). Eliminates QEMU emulation. ARM64 build time: ~40 min → ~10 min.

- **QEMU workflow archived** as `BuildEmAll-QEMU.yml`, manual-trigger only. Use as fallback if native ARM runner is unavailable.

- **Workflow trigger paths expanded** to include `templates/**` so template-only changes trigger a rebuild.

- **Release body injection hardened** — commit message is now written to a temp file via `printf '%s\n' "$COMMIT_MSG"` and consumed with `body_path`, eliminating the `${{ github.event.head_commit.message }}` expression-context injection risk in the release step.

---

## [2.1.0] - 2025-12-30

### Added

- Multi-architecture Docker builds (linux/amd64, linux/arm64) via `BuildEmAll.yml`
- Native ARM build workflow (`BuildEmAll-Fast.yml`) with `ubuntu-24.04-arm`
- ARM runner documentation (`ARM-RUNNERS.md`)
- CLI tool (`microbin` script) for text/file upload and retrieval from the command line
- Docker deployment documentation (`DOCKER.md`) with Traefik, Nginx Proxy Manager, and CloudPanel examples

### Changed

- Base Docker image updated to `debian:bookworm-slim`
- Build caching via registry (`buildcache-linux-amd64`, `buildcache-linux-arm64` tags)
- Multi-arch manifest merged by digest after parallel platform builds

### Fork defaults (vs upstream)

- QR code generation: enabled
- Eternal pastes: enabled
- Default expiry: never
- Telemetry: disabled
- Hash IDs with animal names: enabled
- Client-side and server-side encryption: enabled

---

## [2.0.x] - 2025-11-04

Initial w3K fork from [danielszabo99/microbin](https://github.com/szabodanika/microbin) v2.0.x.

- Forked and published to `w3kllc/microbin` on Docker Hub
- Primary domain set to `microbin.cc`
- Enhanced default configuration baked into source
