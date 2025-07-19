# HTTP API Enhanced Parameters

The HTTP server now supports the same rendering parameters that the Discord bot uses, allowing you to get properly cr## Response Format

All responses are JSON with this structure:

```json
{
  "data": ["base64-encoded-image-or-pdf-data"]
}
```

The response contains a single base64-encoded string representing the rendered output. Decode it to get the actual image or PDF file.

## Adding New Package Repositories

The Typst bot can cache and serve multiple package repositories automatically. Here's how to add a new package repository to the system:

### 1. Configure Package Mapping

Edit the deployment workflow file `.github/workflows/deploy.yml` and update the `PACKAGE_MAPPINGS` environment variable:

```yaml
env:
  # Package name mappings (repo_name:package_name)
  # Add new packages here in the format: "repo1:package1,repo2:package2"
  PACKAGE_MAPPINGS: "2_typst:content,my_package_repo:my_package"
```

**Format**: `"repo_name:package_alias,another_repo:another_alias"`

- `repo_name`: The GitHub repository name (without `premiertutors/`)
- `package_alias`: The friendly name used in Typst imports

### 2. Prepare the Package Repository

Your package repository must:

1. **Have a `typst.toml` file** with version information:

   ```toml
   [package]
   name = "my-package" 
   version = "1.2.3"
   entrypoint = "src/lib.typ"
   authors = ["Your Name"]
   ```

2. **Use semantic version tags** (e.g., `1.2.3`, `2.0.0`) for releases

3. **Have proper Typst package structure**:

   ```text
   my-package-repo/
   ├── typst.toml
   ├── src/
   │   └── lib.typ      # Main entry point
   └── README.md
   ```

### 3. Configure GitHub App Access

The deployment uses a GitHub App for authentication. Ensure the App has access to your package repository:

1. **Repository Access**: Add the new repository to the GitHub App's repository access list
2. **Permissions**: The App needs these permissions on the package repository:
   - **Contents**: Read (to clone/download repository)
   - **Metadata**: Read (for basic repository information)

### 4. Package Repository Settings (Optional)

If you want the package repository to trigger automatic deployments when new versions are released:

1. **Add Repository Dispatch Action** to your package repository's `.github/workflows/` directory:

   ```yaml
   name: Trigger Bot Deployment
   on:
     release:
       types: [published]
   
   jobs:
     trigger-deploy:
       runs-on: ubuntu-latest
       steps:
         - name: Trigger typst-bot deployment
           uses: peter-evans/repository-dispatch@v2
           with:
             token: ${{ secrets.DEPLOY_TRIGGER_TOKEN }}
             repository: premiertutors/typst-bot
             event-type: deploy-release
             client-payload: |
               {
                 "tag_name": "${{ github.event.release.tag_name }}",
                 "release_url": "${{ github.event.release.html_url }}"
               }
   ```

2. **Add the deployment trigger token** as a repository secret:

   - Go to your package repository Settings → Secrets and variables → Actions
   - Add a new secret named `DEPLOY_TRIGGER_TOKEN`
   - Use the same GitHub App token that has access to the typst-bot repository

### 5. Test the Integration

After configuration:

1. **Tag a release** in your package repository (e.g., `git tag 1.0.0 && git push --tags`)
2. **Trigger manual deployment** via GitHub Actions if needed
3. **Verify package is available** by checking the cache directory on the server: `/opt/typst-bot/cache/pt/your_package/version/`

### 6. Using the Package in Typst

Once deployed, users can import your package in Typst code:

```typst
#import "@pt/my_package:1.2.3": *

// Use functions from your package
#my-function("Hello")
```

The package will be automatically available through the bot's HTTP API and Discord integration.

### Example: Adding a Math Package

Let's say you want to add a repository called `typst-math-ext` with the alias `mathext`:

1. **Update workflow**:

   ```yaml
   PACKAGE_MAPPINGS: "2_typst:content,typst-math-ext:mathext"
   ```

2. **Repository structure**:

   ```text
   typst-math-ext/
   ├── typst.toml           # version = "1.0.0"
   ├── src/
   │   └── lib.typ          # Math functions
   └── README.md
   ```

3. **Usage in Typst**:

   ```typst
   #import "@pt/mathext:1.0.0": advanced-integral
   
   $ #advanced-integral(0, infinity, "e^(-x^2)") $
   ```

## Adding Custom Fonts

To add new fonts, simply place font files in the `fonts/` directory and restart the server.

**Supported formats:** `.ttf`, `.otf`, `.ttc`, `.otc`

The server will automatically load all fonts from the directory at startup.

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
