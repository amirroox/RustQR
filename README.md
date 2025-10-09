# RustQR - QR Code CLI Generator

A powerful command-line tool for generating customizable QR codes with advanced styling options, built in Rust for maximum performance.

## Features

- **Custom Colors**: Set foreground and background colors
- **Gradient Support**: Apply color gradients across the QR code
- **Multiple Dot Styles**: Choose between square, circle, or rounded dots
- **Eye Customization**: Customize the three corner "eyes" with different styles
- **Logo Integration**: Add your logo in the center of the QR code
- **Error Correction**: Support for all error correction levels (L, M, Q, H)
- **Multiple Formats**: Export as PNG, JPG, SVG & (webp, tiff, tif, ico, bmp, gif, tga, avif, qoi)
- **Terminal Display**: Preview QR code directly in your terminal
- **Clipboard Support**: Copy output path to clipboard
- **Base64 Encoding**: Optionally encode data before generating QR
- **Interactive Mode**: User-friendly prompts for all options

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo

### Build from Source

```bash
git clone https://github.com/amirroox/RustQR
cd RustQR
cargo build --release
```

The compiled binary will be located at `target/release/RustQR`.

### Install Globally

```bash
cargo install --path .
```

## Usage

### Basic Usage

Generate a simple QR code:

```bash
RustQR --data "https://qrcode.ro-ox.com" --output qr.png
# or Windows
RustQR.exe --data "https://qrcode.ro-ox.com" --output qr.png
```

### Advanced Examples

#### Custom Colors with Gradient

```bash
RustQR --data "https://qrcode.ro-ox.com" \
  --gradient "#ff0000,#0000ff" \
  --bg-color "#ffffff" \
  --size 500 \
  --output gradient-qr.png
```

#### With Logo and Rounded Dots

```bash
RustQR --data "https://qrcode.ro-ox.com" \
  --fg-color "#000000" \
  --bg-color "#ffffff" \
  --dot-style rounded \
  --eye-style circle \
  --logo logo.png \
  --logo-size 0.2 \
  --size 600 \
  --output branded-qr.png
```

#### High Error Correction with Terminal Preview

```bash
RustQR --data "Important Data" \
  --error H \
  --show \
  --output secure-qr.png
```

#### Base64 Encoded Data

```bash
RustQR --data "Secret Message" \
  --encode \
  --output encoded-qr.png
```

### Interactive Mode

For a guided experience with prompts:

```bash
RustQR --interactive
```

This will walk you through all options step by step :)

## Command-Line Options

| Option          | Short | Description                         | Default      |
|-----------------|-------|-------------------------------------|--------------|
| `--data`        | `-d`  | Text or URL to encode               | (required)   |
| `--output`      | `-o`  | Output file path                    | `qrcode.png` |
| `--format`      | `-f`  | Output format (png)                 | `png`        |
| `--bg-color`    |       | Background color (hex: #ffffff)     | `#ffffff`    |
| `--fg-color`    |       | Foreground color (hex: #000000)     | `#000000`    |
| `--gradient`    | `-g`  | Gradient colors (#ff0000,#0000ff)   | -            |
| `--dot-style`   |       | Dot style (square, circle, rounded) | `square`     |
| `--eye-style`   |       | Eye style (square, circle, frame)   | `square`     |
| `--logo`        | `-l`  | Logo file path                      | -            |
| `--logo-size`   |       | Logo size ratio (0.1-0.4)           | `0.2`        |
| `--error`       | `-e`  | Error correction level (L, M, Q, H) | `M`          |
| `--size`        | `-s`  | Image size in pixels                | `300`        |
| `--border`      | `-b`  | Border size (quiet zone)            | `4`          |
| `--show`        |       | Display QR in terminal              | `false`      |
| `--copy`        |       | Copy path to clipboard              | `false`      |
| `--encode`      |       | Base64 encode data                  | `false`      |
| `--version`     | `-v`  | QR version (1-40)                   | auto         |
| `--interactive` | `-i`  | Interactive mode                    | `false`      |

## Styling Options

### Dot Styles

- **square**: Standard square modules (default)
- **circle**: Circular dots for a modern look
- **rounded**: Rounded square corners for a softer appearance

### Eye Styles

- **square**: Standard square eyes (default)
- **circle**: Circular eyes
- **frame**: Hollow frame eyes

### Error Correction Levels

- **L**: ~7% correction capability
- **M**: ~15% correction capability (default)
- **Q**: ~25% correction capability
- **H**: ~30% correction capability (recommended when using logos)

## Examples Gallery

### Example 1: Corporate Branding
```bash
RustQR --data "https://qrcode.ro-ox.com" \
  --fg-color "#003366" \
  --bg-color "#f0f0f0" \
  --dot-style rounded \
  --eye-style frame \
  --logo company-logo.png \
  --size 800 \
  --error H
```

### Example 2: Vibrant Gradient
```bash
RustQR --data "https://qrcode.ro-ox.com" \
  --gradient "#ff6b6b,#4ecdc4" \
  --bg-color "#ffffff" \
  --dot-style circle \
  --size 600
```

### Example 3: Minimalist Black & White
```bash
RustQR --data "https://qrcode.ro-ox.com" \
  --fg-color "#000000" \
  --bg-color "#ffffff" \
  --dot-style square \
  --size 400 \
  --border 2
```

## Tips & Best Practices

1. **Logo Size**: Keep logo size between 0.15-0.25 for optimal scanning
2. **Error Correction**: Use level H when adding logos or using gradients
3. **Contrast**: Ensure sufficient contrast between foreground and background colors
4. **Testing**: Always test QR codes with multiple scanning apps before production use
5. **Size**: For print, use at least 600x600 pixels
6. **Border**: Keep at least 4 modules of quiet zone (white space) around the QR code

## Troubleshooting

### QR Code Not Scanning

- Increase error correction level to Q or H
- Reduce logo size
- Increase overall QR code size
- Ensure sufficient contrast between colors
- Add more border space

### Colors Not Appearing

- Verify hex color format includes '#' prefix
- Check that gradient has exactly 2 colors separated by comma
- Ensure colors have sufficient contrast

### File Not Saving

- Check write permissions in output directory
- Verify output path is valid
- Ensure sufficient disk space

## Development

### Project Structure

```
RustQR/
├── Cargo.toml          # Dependencies and project metadata
├── src/
│   ├── main.rs         # Main application logic and CLI handling
│   └── styles.rs       # Styling functions (dots, eyes, gradients)
└── README.md           # This file
```

### Building for Release

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

## Dependencies

- `qrcode` - QR code generation
- `image` - Image processing and manipulation
- `clap` - Command-line argument parsing
- `dialoguer` - Interactive prompts
- `anyhow` - Error handling
- `base64` - Base64 encoding
- `cli-clipboard` - Clipboard operations
- `csscolorparser` - Color parsing

## License

MIT License - feel free to use this project for any purpose.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Future Features

- [ ] Batch processing
- [ ] QR code reading/decoding
- [ ] More gradient patterns (radial, diagonal)
- [ ] Custom module shapes

## Acknowledgments

This project was inspired by [qrcode.ro-ox.com](https://qrcode.ro-ox.com/) and built to provide similar functionality in a CLI format.

## Contact & Support

For bugs, feature requests, or questions, please open an issue on the project repository.