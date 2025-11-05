# MicroBin - Minimalist Pastebin

A tiny, self-hosted, encrypted pastebin service written in Rust.

## 🚀 Quick Start

```bash
docker run -d -p 8080:8080 w3kllc/microbin:latest
```

Visit `http://localhost:8080` to access MicroBin.

## 🐳 Supported Platforms

This image supports multiple architectures:

- `linux/amd64` - Intel/AMD 64-bit (Desktop, Servers, Cloud)
- `linux/arm64` - ARM 64-bit (Raspberry Pi 4+, Apple Silicon, AWS Graviton)

Docker will automatically pull the correct image for your platform.

## 📋 Usage

### Basic Usage

```bash
docker run -d \
  -p 8080:8080 \
  --name microbin \
  w3kllc/microbin:latest
```

### With Environment Variables

```bash
docker run -d \
  -p 8080:8080 \
  -e MICROBIN_PORT=8080 \
  -e MICROBIN_PUBLIC_PATH=https://paste.example.com \
  --name microbin \
  w3kllc/microbin:latest
```

### With Persistent Storage

```bash
docker run -d \
  -p 8080:8080 \
  -v microbin-data:/app/data \
  --name microbin \
  w3kllc/microbin:latest
```

### Docker Compose

```yaml
version: '3.8'

services:
  microbin:
    image: w3kllc/microbin:latest
    container_name: microbin
    ports:
      - "8080:8080"
    volumes:
      - microbin-data:/app/data
    environment:
      - MICROBIN_PORT=8080
    restart: unless-stopped

volumes:
  microbin-data:
```

## 🔧 Configuration

MicroBin can be configured using environment variables. See the [configuration documentation](https://github.com/w3K-one/microbin#configuration) for all available options.

### Common Environment Variables

- `MICROBIN_PORT` - Port to listen on (default: 8080)
- `MICROBIN_PUBLIC_PATH` - Public URL of your MicroBin instance
- `MICROBIN_BIND` - Address to bind to (default: 0.0.0.0)
- `MICROBIN_HIDE_LOGO` - Hide the logo (default: false)
- `MICROBIN_HIDE_FOOTER` - Hide the footer (default: false)
- `MICROBIN_ENCRYPTION_CLIENT_SIDE` - Enable client-side encryption (default: false)
- `MICROBIN_ENCRYPTION_SERVER_SIDE` - Enable server-side encryption (default: false)

## 📦 Image Tags

- `latest` - Latest stable release
- `v1.x.x` - Specific version tags

## 🏗️ Building

This image is automatically built for multiple architectures using GitHub Actions.

To build locally:

```bash
docker buildx build --platform linux/amd64,linux/arm64 -t microbin:local .
```

## 📚 Links

- [GitHub Repository](https://github.com/w3K-one/microbin)
- [Documentation](https://github.com/w3K-one/microbin#readme)
- [Docker Hub](https://hub.docker.com/r/w3kllc/microbin)

## 📄 License

See the [LICENSE](https://github.com/w3K-one/microbin/blob/master/LICENSE) file in the source repository.

## 🤝 Contributing

Contributions are welcome! Please visit the [GitHub repository](https://github.com/w3K-one/microbin) to report issues or submit pull requests.

---

**Built with ❤️ using Rust**
