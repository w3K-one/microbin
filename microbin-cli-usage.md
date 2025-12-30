# MicroBin Command Line Usage Guide

This guide explains how to create pastes in MicroBin directly from the command line using curl.

## Basic Usage

Create a simple text paste:

```bash
curl -X POST "http://your-microbin-url/upload" \
  -F "content=Your paste content here" \
  -F "expiration=never" \
  -F "privacy=unlisted"
```

## Available Form Fields

### Required Fields

- `content` - The text content to paste

### Optional Fields

- `file` - For file uploads (use `@filename` syntax)
- `expiration` - When the paste expires
  - Options: `1min`, `10min`, `1hour`, `24hour`, `3days`, `1week`, `never`
  - Default: `24hour` (or your configured default)
- `privacy` - Privacy level
  - `public` - Findable by anyone
  - `unlisted` - Only accessible with the URL
  - `readonly` - Viewable by anyone with URL, requires password to edit/delete
  - `private` - Requires password to view (server-side encryption)
  - `secret` - Requires password to view (client-side encryption)
- `burn_after` - Number of reads before deletion (if enabled on server)
  - Options: `1`, `10`, `100`, `1000`, `10000`, `0` (no limit)
- `password_field` - Password for protected pastes (required for `readonly`, `private`, `secret`)
- `syntax_highlight` - Syntax highlighting option (if enabled on server)

## Examples

### Create a Basic Public Paste

```bash
curl -X POST "http://your-microbin-url/upload" \
  -F "content=Hello from the command line!" \
  -F "expiration=24hour" \
  -F "privacy=public"
```

### Create an Unlisted Paste That Never Expires

```bash
curl -X POST "http://your-microbin-url/upload" \
  -F "content=This is a secret message" \
  -F "expiration=never" \
  -F "privacy=unlisted"
```

### Upload a File

```bash
curl -X POST "http://your-microbin-url/upload" \
  -F "file=@/path/to/your/file.txt" \
  -F "expiration=1week" \
  -F "privacy=unlisted"
```

### Create a Read-Only Paste with Password

```bash
curl -X POST "http://your-microbin-url/upload" \
  -F "content=Protected content" \
  -F "expiration=never" \
  -F "privacy=readonly" \
  -F "password_field=mypassword123"
```

### Create a Burn-After-Read Paste

```bash
curl -X POST "http://your-microbin-url/upload" \
  -F "content=This will self-destruct after being read once" \
  -F "expiration=24hour" \
  -F "privacy=unlisted" \
  -F "burn_after=1"
```

### Pipe Content from a Command

```bash
echo "Hello from command line" | curl -X POST "http://your-microbin-url/upload" \
  -F "content=<-" \
  -F "expiration=never" \
  -F "privacy=unlisted"
```

### Pipe Command Output

```bash
ls -la | curl -X POST "http://your-microbin-url/upload" \
  -F "content=<-" \
  -F "expiration=1hour" \
  -F "privacy=unlisted"
```

### Upload System Logs

```bash
cat /var/log/syslog | curl -X POST "http://your-microbin-url/upload" \
  -F "content=<-" \
  -F "expiration=24hour" \
  -F "privacy=private" \
  -F "password_field=logpass123"
```

## Capturing the Paste URL

By default, MicroBin responds with a redirect to the paste page. To capture the final URL:

```bash
curl -X POST "http://your-microbin-url/upload" \
  -F "content=Your content" \
  -F "expiration=never" \
  -F "privacy=unlisted" \
  -w '%{url_effective}' \
  -o /dev/null \
  -s \
  -L
```

### Save URL to Variable

```bash
PASTE_URL=$(curl -X POST "http://your-microbin-url/upload" \
  -F "content=Your content" \
  -F "expiration=never" \
  -F "privacy=unlisted" \
  -w '%{url_effective}' \
  -o /dev/null \
  -s \
  -L)

echo "Paste URL: $PASTE_URL"
```

## Creating a Shell Function

Add this to your `.bashrc` or `.zshrc` for easy pasting:

```bash
paste() {
    local content="${1:-$(cat)}"
    local url="http://your-microbin-url"
    
    curl -X POST "$url/upload" \
      -F "content=$content" \
      -F "expiration=never" \
      -F "privacy=unlisted" \
      -w '%{url_effective}\n' \
      -o /dev/null \
      -s \
      -L
}
```

Usage:

```bash
# From argument
paste "Some text to paste"

# From stdin
echo "Hello" | paste

# From command output
cat file.txt | paste
```

## Creating a Shell Alias

For quick one-liners:

```bash
alias pb='curl -X POST "http://your-microbin-url/upload" -F "content=<-" -F "expiration=never" -F "privacy=unlisted" -w "%{url_effective}\n" -o /dev/null -s -L'
```

Usage:

```bash
echo "Quick paste" | pb
cat file.txt | pb
ls -la | pb
```

## Advanced: File Upload with Metadata

```bash
curl -X POST "http://your-microbin-url/upload" \
  -F "file=@document.pdf" \
  -F "expiration=1week" \
  -F "privacy=private" \
  -F "password_field=secretpass" \
  -F "burn_after=5" \
  -w '%{url_effective}\n' \
  -o /dev/null \
  -s \
  -L
```

## Notes

- If your MicroBin instance has `MICROBIN_HASH_IDS=true`, URLs will use short hash IDs instead of animal names
- If `MICROBIN_ETERNAL_PASTA=true`, the "never" expiration option will truly never expire
- If `MICROBIN_QR=true`, pastes will include QR codes for easy mobile sharing
- Some privacy levels require `MICROBIN_ENCRYPTION_CLIENT_SIDE` or `MICROBIN_ENCRYPTION_SERVER_SIDE` to be enabled
- If `MICROBIN_READONLY=true` and `MICROBIN_UPLOADER_PASSWORD` is set, you'll need to include the uploader password field

## Troubleshooting

### Getting "Incorrect" Response

Make sure your server has the appropriate features enabled:
- For `readonly` privacy: `MICROBIN_ENABLE_READONLY=true`
- For `private` privacy: `MICROBIN_ENCRYPTION_SERVER_SIDE=true`
- For `secret` privacy: `MICROBIN_ENCRYPTION_CLIENT_SIDE=true`

### File Upload Fails

Check your server's file size limits:
- `MICROBIN_MAX_FILE_SIZE_UNENCRYPTED_MB`
- `MICROBIN_MAX_FILE_SIZE_ENCRYPTED_MB`

### URL Not Returned

Ensure you're using the `-L` flag to follow redirects and `-w '%{url_effective}'` to capture the final URL.

## Example Integration Scripts

### Backup Script with Paste

```bash
#!/bin/bash
# Create a backup and paste the log

BACKUP_LOG=$(./backup.sh 2>&1)
PASTE_URL=$(echo "$BACKUP_LOG" | curl -X POST "http://your-microbin-url/upload" \
  -F "content=<-" \
  -F "expiration=1week" \
  -F "privacy=unlisted" \
  -w '%{url_effective}' \
  -o /dev/null \
  -s \
  -L)

echo "Backup completed. Log available at: $PASTE_URL"
```

### Screenshot and Share

```bash
#!/bin/bash
# Take screenshot and upload to MicroBin

TEMP_FILE="/tmp/screenshot_$(date +%s).png"
import "$TEMP_FILE"  # ImageMagick screenshot tool

PASTE_URL=$(curl -X POST "http://your-microbin-url/upload" \
  -F "file=@$TEMP_FILE" \
  -F "expiration=24hour" \
  -F "privacy=unlisted" \
  -w '%{url_effective}' \
  -o /dev/null \
  -s \
  -L)

echo "$PASTE_URL" | xclip -selection clipboard
notify-send "Screenshot uploaded" "$PASTE_URL"
rm "$TEMP_FILE"
```

---

**Remember to replace `http://your-microbin-url` with your actual MicroBin instance URL.**
