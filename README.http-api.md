# HTTP API Enhanced Parameters

The HTTP server now supports the same rendering parameters that the Discord bot uses, allowing you to get properly cropped images with custom backgrounds just like the Discord bot.

## New API Parameters

The `/render` endpoint now accepts these additional optional parameters:

### `theme` (string, optional)
Controls the background color and text color of the rendered image:
- `"dark"` (default): Black background (`rgb(49, 51, 56)`) with light text (`rgb(219, 222, 225)`)
- `"light"`: White background with default black text
- `"transparent"`: No background (transparent)

### `page_size` (string, optional) 
Controls the cropping and sizing behavior:
- `"preview"` (default): Compact size (`width: 300pt, height: auto, margin: 10pt`) - **this gives you cropped images**
- `"auto"`: Auto-sized pages (`width: auto, height: auto, margin: 10pt`)
- `"default"`: Full page size (no size constraints)

### `resolution` (number, optional)
Controls the image resolution/quality:
- Default: `1000.0`
- Higher values = higher resolution images
- Lower values = lower resolution images

## Example Requests

### Get Discord-like cropped images with black background
```json
{
  "code": "Hello, world!",
  "theme": "dark",
  "page_size": "preview"
}
```

### Get cropped images with white background
```json
{
  "code": "= My Document\n\nSome content here.",
  "theme": "light", 
  "page_size": "preview"
}
```

### Get high-resolution cropped images
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

### Discord-style rendering
```bash
curl -X POST http://localhost:8080/render \
  -H "Content-Type: application/json" \
  -d '{
    "code": "= Hello World\n\nThis is *emphasized* text.",
    "theme": "dark",
    "page_size": "preview"
  }'
```

### High-resolution math formula
```bash
curl -X POST http://localhost:8080/render \
  -H "Content-Type: application/json" \
  -d '{
    "code": "$ sum_(i=1)^n i = (n(n+1))/2 $",
    "theme": "dark", 
    "page_size": "preview",
    "resolution": 1500.0
  }'
```

## Key Benefits

1. **Cropped Images**: Use `"page_size": "preview"` to get nicely cropped images instead of full-page renders
2. **Black Background**: Use `"theme": "dark"` to match Discord's appearance
3. **Higher Resolution**: Increase `resolution` for better quality images
4. **Backward Compatibility**: All parameters are optional - existing API calls continue to work

## Comparison with Discord Bot

The Discord bot uses these defaults:
- Theme: `dark` 
- Page Size: `preview`
- Resolution: `1000.0`

To exactly match Discord output, use:
```json
{
  "code": "your typst code here",
  "theme": "dark",
  "page_size": "preview"
}
```
