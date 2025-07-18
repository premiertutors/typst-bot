# Typst Bot HTTP Service

This implementation converts the original typst-bot into an HTTP service that exposes a `/render` endpoint for rendering Typst documents to PNG images.

## Architecture

The refactored codebase consists of:

1. **`worker-lib`** - A shared library containing the core Typst rendering functionality
2. **`http-server`** - An HTTP service that exposes the rendering functionality via REST API
3. **`worker`** - The original binary worker (now using `worker-lib`)

## HTTP API

### POST /render

Renders a Typst document and returns PNG images as base64-encoded strings.

**Request:**

```json
{
  "code": "= Hello World\n\nThis is a Typst document."
}
```

**Response (Success):**

```json
{
  "images": ["iVBORw0KGgoAAAANSUhEUgAA..."], // Array of base64-encoded PNG images
  "more_pages": 0,                           // Number of additional pages not rendered
  "warnings": ""                             // Warning messages from compilation
}
```

**Response (Error):**

```text
Error: unknown variable: invalid-function
   ╭─[/main.typ:1:2]
   │
 1 │ #invalid-function()
   │ 
   │ Help: if you meant to use subtraction, try adding spaces around the minus sign: `invalid - function`
───╯
```

## Building and Running

### Local Development

1. **Install Rust:**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source "$HOME/.cargo/env"
   ```

2. **Build the HTTP server:**

   ```bash
   cargo build -p http-server
   ```

3. **Run the HTTP server:**

   ```bash
   CACHE_DIRECTORY=/tmp/typst-cache cargo run -p http-server
   ```

4. **Test the server:**

   ```bash
   curl -X POST localhost:8080/render \
     -d '{"code":"= Hello World\\n\\nThis is a test document."}' \
     -H 'Content-Type: application/json'
   ```

### Docker

1. **Build and run with Docker Compose:**

   ```bash
   docker-compose -f docker-compose.http.yml up --build
   ```

2. **Test the containerized service:**

   ```bash
   curl -X POST localhost:8080/render \
     -d '{"code":"= Hello World\\n\\nThis is a test document."}' \
     -H 'Content-Type: application/json'
   ```

## Configuration

The HTTP server accepts the following environment variables:

- `CACHE_DIRECTORY` - Directory for caching downloaded packages (default: `/tmp/typst-cache`)

## Files Added/Modified

### New Files

- `crates/worker-lib/` - Shared library with core rendering functionality
- `crates/http-server/` - HTTP service implementation
- `Dockerfile.http` - Docker configuration for the HTTP service
- `docker-compose.http.yml` - Docker Compose configuration
- `test_http_server.sh` - Test script for validation

### Modified Files

- `Cargo.toml` - Added new workspace members
- `crates/worker/` - Refactored to use `worker-lib`

## Implementation Details

### Core Changes

1. **Extracted Shared Library (`worker-lib`):**
   - Moved `render`, `sandbox`, and `diagnostic` modules from `worker` to `worker-lib`
   - Made `Sandbox` cloneable for use in HTTP service
   - Added proper error handling for HTTP context

2. **HTTP Service (`http-server`):**
   - Built with Actix Web framework
   - Handles POST requests to `/render` endpoint
   - Converts binary PNG data to base64 for JSON response
   - Proper error handling with HTTP status codes

3. **Docker Support:**
   - Multi-stage Docker build for optimized image size
   - Proper environment variable configuration
   - Exposed port 8080 for HTTP traffic

### Error Handling

The service properly handles various error conditions:

- **Compilation errors** - Returns detailed diagnostic messages
- **Invalid JSON** - Returns HTTP 400 Bad Request
- **Server errors** - Returns HTTP 500 Internal Server Error

### Performance Considerations

- **Shared sandbox** - Reuses font loading and library initialization across requests
- **Memory management** - Uses `comemo::evict()` for cache management
- **Async processing** - Uses `web::block()` for CPU-intensive rendering

## Testing

Run the included test script to validate functionality:

```bash
./test_http_server.sh
```

The test covers:

1. Valid Typst document rendering
2. Error handling for invalid code
3. Empty document handling

## Usage Examples

### Simple Document

```bash
curl -X POST localhost:8080/render \
  -d '{"code":"= My Document\\n\\nHello, *World*!"}' \
  -H 'Content-Type: application/json'
```

### Math Document

```bash
curl -X POST localhost:8080/render \
  -d '{"code":"$ x = (-b ± sqrt(b^2 - 4a c)) / (2a) $"}' \
  -H 'Content-Type: application/json'
```

### With Packages

```bash
curl -X POST localhost:8080/render \
  -d '{"code":"#import \\"@preview/cetz:0.2.2\\": canvas, draw"}' \
  -H 'Content-Type: application/json'
```
