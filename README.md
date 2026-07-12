# mdx

A simple command-line tool that converts Markdown files into styled HTML.

## What it does

Reads a `.md` file and converts it into a clean, styled HTML page. Supports:

- Headings (`#`, `##`, `###`)
- **Bold** and *italic* text
- `Inline code`
- Lists
- Blockquotes
- Horizontal rules

## Usage

```bash
cargo run -- yourfile.md
```

This generates `output.html` in the project folder. Open it in your browser to view the result.

## Example

Input (`test.md`):
```markdown
# Hello World
This is **bold** and this is *italic*.
```

Output (`output.html`):
```html
<h1>Hello World</h1>
<p>This is <b>bold</b> and this is <i>italic</i>.</p>
```

## Note

Built and tested on Linux (Ubuntu). To run on another OS, clone the repo and build it there with `cargo build --release` — Rust will compile a native binary for your platform.

## Built with

Rust, no external dependencies.
