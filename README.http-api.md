# HTTP API Enhanced Parameters

The HTTP server now supports the same rendering parameters that the Discord bot uses, allowing you to get properly cropped images with custom backgrounds just like the Discord bot. Additionally, it supports PDF output for vector-based, lossless documents.

## Getting Started

### Running the Server Locally

1. **Build the HTTP server:**
   ```bash
   cargo build -p http-server
   ```

2. **Start the server:**
   ```bash
   ./target/debug/http-server
   ```
   The server will start on `http://localhost:8080`

### Testing Output

You can test the API and save outputs directly:

```bash
# Generate and save a PNG
curl -X POST http://localhost:8080/render \
  -H "Content-Type: application/json" \
  -d '{"code": "Hello World!", "theme": "dark"}' \
  | jq -r '.data[0]' | base64 -d > output.png

# Generate and save a PDF  
curl -X POST http://localhost:8080/render \
  -H "Content-Type: application/json" \
  -d '{"code": "Hello World!", "format": "pdf"}' \
  | jq -r '.data[0]' | base64 -d > output.pdf
```

## API Parameters

The `/render` endpoint accepts these optional parameters:

### `theme` (string, optional)
Controls the background color and text color of the rendered output:
- `"dark"` (default): Black background (`rgb(49, 51, 56)`) with light text (`rgb(219, 222, 225)`)
- `"light"`: White background with default black text
- `"transparent"`: No background (transparent)

### `page_size` (string, optional) 
Controls the cropping and sizing behavior:
- `"preview"` (default): Compact size (`width: 300pt, height: auto, margin: 10pt`) - **this gives you cropped output**
- `"auto"`: Auto-sized pages (`width: auto, height: auto, margin: 10pt`)
- `"default"`: Full page size (no size constraints)

### `format` (string, optional)
Controls the output format:
- `"png"` (default): Raster image format
- `"pdf"`: Vector-based PDF format (lossless, scalable)

### `resolution` (number, optional)
Controls the image resolution/quality for PNG output:
- Default: `1000.0`
- Higher values = higher resolution images
- Lower values = lower resolution images
- **Note**: This parameter is ignored for PDF format since PDFs are vector-based

## Example Requests

### Get Discord-like cropped images with black background

```json
{
  "code": "Hello, world!",
  "theme": "dark",
  "page_size": "preview"
}
```

### Get PDF output with dark theme

```json
{
  "code": "= My Document\n\nSome content here.",
  "format": "pdf",
  "theme": "dark", 
  "page_size": "preview"
}
```

### Get high-resolution PNG images

```json
{
  "code": "$x^2 + y^2 = z^2$",
  "theme": "dark",
  "page_size": "preview",
  "resolution": 2000.0
}
```

### Get transparent background (for overlays)

```json
{
  "code": "#text(fill: red)[Important!]",
  "theme": "transparent",
  "page_size": "auto"
}
```

## curl Examples

### Discord-style PNG rendering

```bash
curl -X POST http://localhost:8080/render \
  -H "Content-Type: application/json" \
  -d '{
    "code": "= Hello World\n\nThis is *emphasized* text.",
    "theme": "dark",
    "page_size": "preview"
  }'
```

### Generate and save a PDF document

```bash
curl -X POST http://localhost:8080/render \
  -H "Content-Type: application/json" \
  -d '{
    "code": "= Mathematical Document\n\n$ integral_0^infinity e^(-x^2) d x = sqrt(pi)/2 $",
    "format": "pdf",
    "theme": "dark",
    "page_size": "preview"
  }' | jq -r '.data[0]' | base64 -d > math.pdf
```

### High-resolution math formula PNG

```bash
curl -X POST http://localhost:8080/render \
  -H "Content-Type: application/json" \
  -d '{
    "code": "$ sum_(i=1)^n i = (n(n+1))/2 $",
    "theme": "dark", 
    "page_size": "preview",
    "resolution": 1500.0
  }' | jq -r '.data[0]' | base64 -d > formula.png
```

## Response Format

The API returns a JSON response with the following structure:

```json
{
  "data": ["base64-encoded-output-data"],
  "more_pages": 0,
  "warnings": "",
  "format": "png"
}
```

- `data`: Array of base64-encoded output (PNG images or single PDF)
- `more_pages`: Number of additional pages (if any)
- `warnings`: Any compilation warnings
- `format`: The output format used ("png" or "pdf")

## Key Benefits

1. **Cropped Output**: Use `"page_size": "preview"` to get nicely cropped output instead of full-page renders
2. **Dark Theme**: Use `"theme": "dark"` to match Discord's appearance  
3. **PDF Support**: Use `"format": "pdf"` for vector-based, lossless documents
4. **Higher Resolution**: Increase `resolution` for better quality PNG images
5. **Backward Compatibility**: All parameters are optional - existing API calls continue to work

## Format Comparison

- **PNG**: Raster format, supports resolution parameter, good for embedding in web pages
- **PDF**: Vector format, resolution-independent, smaller file sizes, perfect for documents and printing

## Comparison with Discord Bot

The Discord bot uses these defaults:

- Theme: `dark`
- Page Size: `preview`
- Resolution: `1000.0`
- Format: `png`

To exactly match Discord output, use:

```json
{
  "code": "your typst code here",
  "theme": "dark",
  "page_size": "preview"
}
```
