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

1. Download `mdx-linux-x86_64` from the [Releases](../../releases) page
2. Make it executable:
```bash
   chmod +x mdx-linux-x86_64
```

### Option A: Convert your own Markdown file
Place your `.md` file in the same folder as the binary, then run:
```bash
./mdx-linux-x86_64 yourfile.md
```
Or point to it using a full path from anywhere:
```bash
./mdx-linux-x86_64 /path/to/yourfile.md
```

### Option B: Try it on the included sample file
This repo includes a sample `test.md`. Download it into the same folder as the binary:
```bash
wget https://raw.githubusercontent.com/Tylerr-D/mdx/main/test.md
```
Then run:
```bash
./mdx-linux-x86_64 test.md
```
### Getting the output
Either option generates a file called `output.html` in the same folder you ran the command from.

To view it rendered in a browser:
```bash
xdg-open output.html
```
(or just double-click `output.html` in your file manager)

To quickly check the raw HTML without leaving the terminal:
```bash
cat output.html
```

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
