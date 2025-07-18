# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v1.2.0] - 2025-07-18

### Added

#### 🆕 PDF Output Support
- **Vector-based PDF generation** using `typst-pdf` dependency
- **Resolution-independent** PDF output for professional document generation
- **Lossless format** perfect for printing and document sharing
- PDF format automatically ignores resolution parameter (vector-based)

#### 🎨 Enhanced HTTP API Parameters
- **`format` parameter**: Choose between `"png"` (default) or `"pdf"` output
- **`theme` parameter**: Control appearance with `"dark"` (default), `"light"`, or `"transparent"`
- **`page_size` parameter**: Smart cropping with `"preview"` (default), `"auto"`, or `"default"`
- **`resolution` parameter**: High-quality PNG rendering (ignored for PDF)

#### 🔧 Discord Bot Feature Parity
- **Discord-style cropping**: `page_size: "preview"` matches Discord bot behavior
- **Dark theme backgrounds**: `theme: "dark"` provides black backgrounds like Discord
- **Content-aware sizing**: Typst dynamically adjusts page dimensions to content
- **Backward compatibility**: All new parameters are optional with sensible defaults

#### 📚 Comprehensive Documentation
- **Updated README.http-api.md** with detailed parameter documentation
- **Practical examples** showing PNG vs PDF usage
- **Local development guide** for testing the API
- **curl examples** with file output for easy testing
- **Response format documentation** including new `format` field

### Changed

#### 🔄 Core Architecture Improvements
- **Enhanced `render_with_format` function** supporting both PNG and PDF output
- **New `OutputFormat` enum** for clean format handling
- **Improved parameter validation** with graceful defaults
- **Base64 encoding** for both PNG and PDF data transport

#### 🏗️ Code Structure
- **Extended HTTP server** (`crates/http-server/src/main.rs`) with comprehensive parameter handling
- **Enhanced worker library** (`crates/worker-lib/src/render.rs`) with dual-format rendering
- **Updated dependencies** (`crates/worker-lib/Cargo.toml`) with `typst-pdf = "0.13"`
- **Improved error handling** for invalid parameters and compilation errors

### Fixed

#### 🚀 Deployment Infrastructure
- **Fixed "Repository not found" error** in GitHub Actions deployment
- **Removed SSH key dependency** in favor of HTTPS token authentication
- **Simplified SSH connection** (removed `-A` flag)
- **Updated deployment workflow** to use `GHCR_PAT` token for git operations
- **Streamlined deployment process** with more reliable authentication

#### 🛡️ API Robustness
- **Graceful parameter handling**: Invalid formats default to PNG
- **Smart resolution handling**: Resolution parameter ignored for PDF (as documented)
- **Improved error messages** for better debugging
- **Consistent response format** across all parameter combinations

### Technical Details

#### 🏷️ New Dependencies
```toml
typst-pdf = "0.13"  # Vector-based PDF generation
```

#### 🔌 API Response Format
```json
{
  "data": ["base64-encoded-output-data"],
  "more_pages": 0,
  "warnings": "",
  "format": "png"  // NEW: Indicates actual output format used
}
```

#### 📐 Page Size Behavior
- **`"preview"`**: `width: 300pt, height: auto, margin: 10pt` (Discord-style cropping)
- **`"auto"`**: `width: auto, height: auto, margin: 10pt` (tight content bounds)  
- **`"default"`**: Standard page size, no constraints (full page)

#### 🎨 Theme Options
- **`"dark"`**: `rgb(49, 51, 56)` background, `rgb(219, 222, 225)` text
- **`"light"`**: White background, default black text
- **`"transparent"`**: No background fill

### Usage Examples

#### Discord-Style Rendering
```bash
curl -X POST http://localhost:8080/render \
  -H "Content-Type: application/json" \
  -d '{"code": "Hello World!", "theme": "dark", "page_size": "preview"}'
```

#### PDF Document Generation
```bash
curl -X POST http://localhost:8080/render \
  -H "Content-Type: application/json" \
  -d '{"code": "= Document\n\nContent here", "format": "pdf"}' \
  | jq -r '.data[0]' | base64 -d > document.pdf
```

#### High-Resolution PNG
```bash
curl -X POST http://localhost:8080/render \
  -H "Content-Type: application/json" \
  -d '{"code": "Math: $x^2$", "resolution": 2000.0}' \
  | jq -r '.data[0]' | base64 -d > math.png
```

### Breaking Changes
- None. All changes are backward compatible with existing API usage.

### Migration Guide
- **Existing users**: No changes required, all new parameters are optional
- **New users**: Use `format: "pdf"` for document generation, `theme: "dark"` for Discord-style appearance

---

## Previous Versions

### [v1.1.4] and earlier
- Basic PNG rendering functionality
- Discord bot implementation
- Core Typst compilation features
- HTTP server foundation

---

## Development

### Running Locally
```bash
# Build and start the server
cargo build -p http-server
./target/debug/http-server

# Test the API
curl -X POST http://localhost:8080/render \
  -H "Content-Type: application/json" \
  -d '{"code": "Hello World!", "format": "pdf"}' \
  | jq -r '.data[0]' | base64 -d > test.pdf
```

### Contributing
- All new features maintain backward compatibility
- PDF output uses vector rendering (no resolution parameter)
- Theme and page_size parameters work across both PNG and PDF formats
- Error handling provides helpful defaults and messages
