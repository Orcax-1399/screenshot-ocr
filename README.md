# Screenshot OCR

A personal solution for quick OCR text extraction in niri Wayland compositor via keybindings. Written in Rust for fast startup and minimal overhead.

## Background

This tool was created to solve a specific need: quickly extract text from screenshots in a niri environment using a simple keybinding. It captures a screen area, sends it to the GLM OCR API, and copies the extracted text directly to the clipboard with a notification.

## Features

- Fast Rust implementation with ~50ms startup time
- Native Wayland/niri integration via grim and slurp
- Automatic clipboard integration through wl-copy
- D-Bus notifications for status feedback
- Provider-based architecture for extensibility
- Good support for Chinese, English, and mixed-language text
- Preserves markdown formatting from source images

## Requirements

- Rust 1.70+ (for building)
- Wayland compositor (tested with niri)
- `grim` - Screenshot capture tool
- `slurp` - Area selection tool
- `wl-copy` - Clipboard manager
- GLM API key from [BigModel](https://open.bigmodel.cn/)

## Installation

```bash
git clone https://github.com/YOUR_USERNAME/screenshot-ocr.git
cd screenshot-ocr
cargo build --release
cp target/release/screenshot-ocr ~/.local/bin/
```

## Configuration

Create `~/.config/screenshot-ocr/config.toml`:

```toml
api_key = "your_glm_api_key_here"

[image]
max_edge = 1920
jpeg_quality = 85

[api]
endpoint = "https://open.bigmodel.cn/api/paas/v4/layout_parsing"
model = "glm-ocr"
```

Set secure permissions:
```bash
chmod 600 ~/.config/screenshot-ocr/config.toml
```

Alternatively, use the `GLM_API_KEY` environment variable.

## Usage

### Command Line

```bash
# Select area and extract text
grim -g "$(slurp)" - | screenshot-ocr

# Full screen OCR
grim - | screenshot-ocr
```

### Niri Keybinding

Add to `~/.config/niri/config.kdl`:

```kdl
Mod+Shift+O hotkey-overlay-title="Screenshot OCR" {
    spawn-sh "grim -g \"$(slurp)\" - | /home/YOUR_USERNAME/.local/bin/screenshot-ocr";
}
```

Reload niri config:
```bash
niri msg action load-config-file
```

### Workflow

1. Press `Mod+Shift+O`
2. Select screen area with mouse
3. Wait 1-3 seconds for processing
4. Notification shows extraction status
5. Paste text with `Ctrl+V`

## Architecture

Provider-based design for supporting multiple OCR backends:

```
src/
├── main.rs              # Entry point
├── config.rs            # Configuration loading
├── clipboard.rs         # wl-copy integration
├── notifier.rs          # D-Bus notifications
└── provider/
    ├── mod.rs           # OcrProvider trait
    └── glm.rs           # GLM OCR implementation
```

To add a new OCR provider, implement the `OcrProvider` trait:

```rust
pub trait OcrProvider {
    fn extract_text(&self, image_data: &[u8]) -> Result<String>;
}
```

## Troubleshooting

### Keybinding not working

Ensure you're using the full path to the binary in niri config:
```kdl
spawn-sh "grim -g \"$(slurp)\" - | /home/YOUR_USERNAME/.local/bin/screenshot-ocr";
```

### Check for errors

View error output:
```bash
grim -g "$(slurp)" - | screenshot-ocr
```

Monitor niri logs:
```bash
journalctl --user -u niri -f
```

## Performance

- Binary size: ~1.5 MB (optimized with LTO and strip)
- Startup time: <50ms
- Memory usage: Minimal, short-lived process
- API latency: 1-3 seconds (network dependent)

## License

MIT

## Credits

- OCR powered by [GLM-4V](https://open.bigmodel.cn/)
- Built for [niri](https://github.com/YaLTeR/niri) Wayland compositor
