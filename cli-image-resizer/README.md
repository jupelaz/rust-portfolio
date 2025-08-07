# cli-image-resizer

Command-line application for resizing images using Rust.

## Usage

```bash
cargo run -- input.jpg output.jpg --width 200 --height 200
```

- Required arguments: `input`, `output`, `--width`, and `--height`.

## Dependencies

- `clap` for parsing arguments.
- `image` for image manipulation.