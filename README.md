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

## Theme: No Internet

mdx fits No Internet because it never touches the network. It reads a markdown file off disk, converts it with plain Rust standard library code, and writes the html locally. You could turn off your internet and it would run exactly the same, since it completely works on your machine

## Running the release binary

Grab the binary for your OS from [Releases](https://github.com/Tylerr-D/mdx/releases):
- Linux: `mdx-linux-x86_64`
- Windows: `mdx.exe`

Linux, make it executable first:
```bash
chmod +x mdx-linux-x86_64
```

### Option A: Convert your own file
```bash
./mdx-linux-x86_64 yourfile.md      # Linux
mdx.exe yourfile.md                 # Windows
```

### Option B: Use the included sample from me
```bash
wget https://raw.githubusercontent.com/Tylerr-D/mdx/main/test.md
```
(or grab it directly: https://raw.githubusercontent.com/Tylerr-D/mdx/main/test.md)

Then run as above with `test.md`.

### Output
Writes `output.html` to the same folder.
```bash
xdg-open output.html   # Linux
cat output.html        # either OS, raw HTML in terminal
```
Windows: just open `output.html` directly.

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
