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

## Running the release binary

1. Download `mdx-linux-x86_64` from the [Releases](../../releases) page
2. Make it executable:
```bash
   chmod +x mdx-linux-x86_64
```
3. Run it on a markdown file:
```bash
   ./mdx-linux-x86_64 yourfile.md
```
4. Open the generated `output.html` in your browser

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
