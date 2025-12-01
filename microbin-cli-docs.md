# MicroBin CLI Script Documentation

A command-line tool for uploading content to your MicroBin instance.

## Installation

1. Download the script:
```bash
curl -O https://raw.githubusercontent.com/yourusername/yourrepo/main/microbin
```

2. Make it executable:
```bash
chmod +x microbin
```

3. Move to your PATH (optional):
```bash
sudo mv microbin /usr/local/bin/
```

## Quick Start

```bash
# Simple text paste
microbin "Hello, world!"

# Pipe content
echo "Hello from pipe" | microbin

# Upload a file
microbin -f document.pdf

# Get raw URL for curl/wget usage
cat script.sh | microbin -r
```

## Usage

```
microbin [options] "content to paste"
<command> | microbin [options]
microbin -f <file> [options]
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-f <file>` | Upload a file instead of text | - |
| `-e <val>` | Set expiration time | `never` |
| `-p <val>` | Set privacy level | `unlisted` |
| `-P <pass>` | Set password for protected pastes | - |
| `-b <num>` | Burn after N reads (0 = no limit) | - |
| `-c` | Copy URL to clipboard | `false` |
| `-r` | Return raw paste URL | `false` |
| `-h` | Show help message | - |

### Expiration Options

- `1min` - 1 minute
- `10min` - 10 minutes
- `1hour` - 1 hour
- `24hour` - 24 hours
- `3days` - 3 days
- `1week` - 1 week
- `never` - Never expires (if eternal pasta is enabled)

### Privacy Levels

- `public` - Listed publicly, anyone can view/edit/delete
- `unlisted` - Not listed, anyone with URL can view/edit/delete
- `readonly` - Unlisted, requires password to edit/delete
- `private` - Requires password to view (server-side encryption)
- `secret` - Requires password to view (client-side encryption)

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `MICROBIN_URL` | Your MicroBin server URL | `https://microbin.cc` |

Set in your shell profile:
```bash
export MICROBIN_URL="https://your-server.com"
```

## Examples

### Basic Text Paste

```bash
# Simple string
microbin "This is my paste content"

# Multi-line with echo
echo -e "Line 1\nLine 2\nLine 3" | microbin

# Here document
microbin << 'EOF'
This is a multi-line
paste with several
lines of content
EOF
```

### File Uploads

```bash
# Upload a single file
microbin -f document.pdf

# Upload with expiration
microbin -f screenshot.png -e 24hour

# Upload with password protection
microbin -f secrets.txt -p private -P "mypassword123"
```

### Command Output

```bash
# System information
uname -a | microbin

# Directory listing
ls -la | microbin -e 1hour

# Log file
tail -n 100 /var/log/syslog | microbin -p unlisted

# Command with error output
docker ps -a 2>&1 | microbin
```

### Raw URLs for Scripts

The `-r` flag returns the raw paste URL, perfect for scripts that need to fetch the content:

```bash
# Create paste and get raw URL
PASTE_URL=$(echo "Hello World" | microbin -r)

# Fetch content later
curl -s "$PASTE_URL"

# Or with wget
wget -qO- "$PASTE_URL"
```

Example workflow:
```bash
# Server 1: Create and share configuration
CONFIG_URL=$(cat config.yaml | microbin -r -e 1hour)
echo "Config URL: $CONFIG_URL"

# Server 2: Download configuration
curl -s "$CONFIG_URL" > config.yaml
```

### Burn After Reading

```bash
# Self-destruct after 1 read
echo "Secret message" | microbin -b 1 -p unlisted

# Self-destruct after 10 reads
microbin -f data.csv -b 10 -e 24hour
```

### Clipboard Integration

```bash
# Copy URL to clipboard automatically
microbin "Important info" -c

# With raw URL
cat script.sh | microbin -r -c
```

### Protected Pastes

```bash
# Read-only (others can view but not edit without password)
microbin "Public info" -p readonly -P "editpass"

# Private (server-side encryption, password required to view)
microbin "Sensitive data" -p private -P "viewpass"

# Secret (client-side encryption, most secure)
echo "Top secret" | microbin -p secret -P "strongpass"
```

### Complex Examples

```bash
# Upload log with multiple options
tail -f /var/log/app.log | head -n 50 | \
  microbin -e 1week -p private -P "logpass" -c

# Backup script output with burn after
./backup.sh 2>&1 | \
  microbin -b 5 -e 3days -c

# Share compressed directory
tar czf - ~/myproject | \
  microbin -f - -e 24hour -c

# Create a temporary code snippet
microbin -p unlisted -e 1hour -c << 'CODE'
#!/bin/bash
echo "Hello from shared script"
# Your code here
CODE
```

## Shell Functions

Add these to your `~/.bashrc` or `~/.zshrc`:

### Simple Paste Function

```bash
pb() {
    microbin "$@"
}

# Usage
pb "Quick paste"
echo "Hello" | pb
```

### Advanced Paste Function with Defaults

```bash
# Quick unlisted paste
pbq() {
    microbin -p unlisted -e 1hour "$@"
}

# Secure paste (private with password prompt)
pbs() {
    read -sp "Password: " pass
    echo
    microbin -p private -P "$pass" "$@"
}

# Temporary paste (burns after 1 read)
pbt() {
    microbin -b 1 -e 1hour "$@"
}

# Raw paste for scripting
pbr() {
    microbin -r "$@"
}
```

### Screenshot and Paste

```bash
# Take screenshot and upload (requires scrot or ImageMagick)
pbss() {
    local file="/tmp/screenshot_$(date +%s).png"
    scrot -s "$file"  # Or: import "$file" for ImageMagick
    microbin -f "$file" -e 24hour -c
    rm "$file"
}
```

## Aliases

```bash
# Quick paste
alias pb='microbin'

# Raw paste
alias pbr='microbin -r'

# Clipboard paste
alias pbc='microbin -c'

# Temporary paste (1 hour)
alias pbt='microbin -e 1hour'

# Private paste
alias pbp='microbin -p private'
```

## Integration Examples

### Deployment Script

```bash
#!/bin/bash
# Deploy and share logs

echo "Starting deployment..."
DEPLOY_LOG=$(./deploy.sh 2>&1 | tee /dev/tty)

# Share deployment log
LOG_URL=$(echo "$DEPLOY_LOG" | microbin -e 1week -c)
echo ""
echo "Deployment log: $LOG_URL"

# Send to Slack/Discord
curl -X POST "https://hooks.slack.com/..." \
  -d "{\"text\": \"Deployment complete: $LOG_URL\"}"
```

### Automated Backup Reporting

```bash
#!/bin/bash
# Backup with status report

BACKUP_RESULT=$(./run-backup.sh 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
    STATUS="✅ SUCCESS"
else
    STATUS="❌ FAILED"
fi

REPORT_URL=$(cat << EOF | microbin -e 3days -c
Backup Report - $(date)
Status: $STATUS
Exit Code: $EXIT_CODE

$BACKUP_RESULT
EOF
)

echo "Backup report: $REPORT_URL"
```

### Share Configuration Between Servers

```bash
# Server 1: Export and share
kubectl get configmap my-config -o yaml | \
  microbin -r -e 1hour > /tmp/config_url.txt

# Send URL to Server 2 (via SSH, etc.)
scp /tmp/config_url.txt user@server2:/tmp/

# Server 2: Import configuration
CONFIG_URL=$(cat /tmp/config_url.txt)
curl -s "$CONFIG_URL" | kubectl apply -f -
```

### Error Reporting

```bash
#!/bin/bash
# Application with error reporting

run_app() {
    ./my-application 2>&1
}

# Run and capture output
OUTPUT=$(run_app)
EXIT_CODE=$?

if [ $EXIT_CODE -ne 0 ]; then
    ERROR_URL=$(echo "$OUTPUT" | microbin -p private -P "errorpass" -b 1)
    echo "Application failed! Error report: $ERROR_URL"
    exit 1
fi
```

## Troubleshooting

### Script Not Found

```bash
# Make sure it's executable
chmod +x microbin

# Check if it's in your PATH
which microbin

# Or run with full path
./microbin "test"
```

### SSL Certificate Errors

If you get certificate errors, make sure you're NOT using the `-k` flag in curl. Install proper certificates instead:

```bash
# Ubuntu/Debian
sudo apt-get install ca-certificates

# CentOS/RHEL
sudo yum install ca-certificates
```

### Clipboard Not Working

Install the appropriate clipboard tool:

```bash
# X11 (Linux)
sudo apt-get install xclip

# Wayland (Linux)
sudo apt-get install wl-clipboard

# macOS has pbcopy by default
```

### Server URL Issues

Check your environment variable:

```bash
echo $MICROBIN_URL

# Set it if not configured
export MICROBIN_URL="https://your-server.com"

# Or use inline
MICROBIN_URL="https://other-server.com" microbin "test"
```

### Permission Denied on File Upload

```bash
# Check file permissions
ls -la yourfile.txt

# Make sure you have read access
chmod +r yourfile.txt
```

## Security Notes

1. **Never disable SSL verification** - The script does not use `-k` flag
2. **Use private/secret privacy** for sensitive data
3. **Set short expirations** for temporary content
4. **Use burn after** for one-time secrets
5. **Strong passwords** for protected pastes
6. **Be cautious with public pastes** - they're visible to everyone

## Server Requirements

Your MicroBin server should have these features enabled (in `.env`):

```bash
# For eternal pastes
MICROBIN_ETERNAL_PASTA=true

# For read-only pastes
MICROBIN_ENABLE_READONLY=true

# For private pastes
MICROBIN_ENCRYPTION_SERVER_SIDE=true

# For secret pastes
MICROBIN_ENCRYPTION_CLIENT_SIDE=true

# For burn after
MICROBIN_ENABLE_BURN_AFTER=true

# For shorter URLs
MICROBIN_HASH_IDS=true
```

## Tips and Tricks

### Create Paste and Open in Browser

```bash
paste_and_open() {
    local url=$(microbin "$@")
    echo "$url"
    
    # Open in default browser
    if command -v xdg-open &> /dev/null; then
        xdg-open "$url"
    elif command -v open &> /dev/null; then
        open "$url"
    fi
}
```

### Paste from Clipboard

```bash
# Linux (X11)
xclip -o | microbin

# macOS
pbpaste | microbin

# Wayland
wl-paste | microbin
```

### Monitor and Paste

```bash
# Watch a file and paste changes
watch -n 60 'tail -n 20 /var/log/app.log | microbin -e 1hour'
```

### Batch File Upload

```bash
# Upload multiple files
for file in *.log; do
    echo "Uploading $file..."
    url=$(microbin -f "$file" -e 1week)
    echo "$file: $url"
done
```

## License

This script is provided as-is for use with MicroBin instances.

## Contributing

Found a bug or have a suggestion? Feel free to submit issues or pull requests!

---

**Server URL**: Replace `https://microbin.cc` with your MicroBin instance URL using the `MICROBIN_URL` environment variable.
