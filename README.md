# md-forge 🔥

A blazingly fast, lightweight Markdown to HTML converter built entirely in Rust. Convert your Markdown documents to clean, semantic HTML with zero dependencies.

## Features

- 🚀 **Fast & Lightweight**: Pure Rust implementation with no external dependencies
- 📝 **Comprehensive Markdown Support**: Headers, lists, blockquotes, code blocks, and inline formatting
- 🔗 **Link Processing**: Automatic conversion of Markdown links to HTML anchors
- 💪 **Bold & Italic**: Support for `**bold**`, `*italic*`, and `***bold italic***` text
- 📋 **Lists**: Both ordered (numbered) and unordered (bullet) lists
- 💬 **Blockquotes**: Clean blockquote rendering with nested formatting support
- 🖥️ **Code Blocks**: Fenced code blocks with language specification support
- 🎯 **Headers**: All six levels of headers (H1-H6)

## Installation

### From crates.io
```bash
cargo install md-forge
```

### From source
```bash
git clone https://github.com/yourusername/md-forge
cd md-forge
cargo build --release
```

## Usage

### Command Line
```bash
md-forge input.md output.html
```

### Example
```bash
# Convert a README.md to HTML
md-forge README.md README.html

# Convert documentation
md-forge docs/guide.md public/guide.html
```

## Supported Markdown Features

### Headers
```markdown
# H1 Header
## H2 Header
### H3 Header
#### H4 Header
##### H5 Header
###### H6 Header
```

### Text Formatting
```markdown
**Bold text**
*Italic text*
***Bold and italic text***
```

### Lists
```markdown
- Unordered list item
- Another item
- Third item

1. Ordered list item
2. Second item
3. Third item
```

### Links
```markdown
[Link text](https://example.com)
[Rust Documentation](https://doc.rust-lang.org/)
```

### Blockquotes
```markdown
> This is a blockquote
> It can span multiple lines
```

### Code Blocks
````markdown
```
Plain code block
```

```rust
// Code block with language specification
fn main() {
    println!("Hello, world!");
}
```