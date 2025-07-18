use std::error::Error;
use std::fs;
use std::env;

#[derive(Debug)]
enum Block {
    Header(u8, String),
    Paragraph(String),
    UnorderedList(Vec<String>),
    OrderedList(Vec<String>),
    Blockquote(String),
    CodeBlock(String),

}
fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}",e);
        std::process::exit(1);
    } 
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
       return Err("Usage: md-forge <input.md> <output.html>".into());
        
    }
    let input_filename = &args[1];
    let output_filename = &args[2];

    println!("Reading from {}...", input_filename);
    let markdown_content = fs::read_to_string(input_filename)?;

    let blocks = tokenize(&markdown_content);

    let html_content = render_blocks_to_html(blocks);

    println!("Writing to {}...", output_filename);
    fs::write(output_filename, html_content)?;

    println!("Conversion Successful!");
    Ok(())
}

fn tokenize(markdown: &str) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut current_block_lines: Vec<&str> = Vec::new();

    for line in markdown.lines() {
        if line.trim().is_empty() {
            if !current_block_lines.is_empty() {
                blocks.push(parse_block(current_block_lines));
                current_block_lines = Vec::new();
            }
        } else {
            current_block_lines.push(line);
        }
    }

    if !current_block_lines.is_empty() {
        blocks.push(parse_block(current_block_lines));
    }
    blocks
}

fn parse_block(lines: Vec<&str>) -> Block {
    let first_line = lines[0].trim();

    if first_line.starts_with("###### ") {
        Block::Header(6, first_line[7..].to_string())
    } else if first_line.starts_with("##### ") {
        Block::Header(5, first_line[6..].to_string())
    } else if first_line.starts_with("#### ") {
        Block::Header(4, first_line[5..].to_string())
    } else if first_line.starts_with("### ") {
        Block::Header(3, first_line[4..].to_string())
    } else if first_line.starts_with("## ") {
        Block::Header(2, first_line[3..].to_string())
    } else if first_line.starts_with("# ") {
        Block::Header(1, first_line[2..].to_string())
    } else if first_line.starts_with("```") {
        let content = if lines.len() > 1 {
            lines[1..lines.len() - 1].join("\n")
        } else {
            "".to_string()
        };
        Block::CodeBlock(content)
    } else if first_line.starts_with("- ") {
        let items = lines
            .iter()
            .map(|line| line.trim_start_matches("- ").trim().to_string())
            .collect();
        Block::UnorderedList(items)
    } else if first_line.starts_with("1. ") {
        let items = lines
            .iter()
            .filter_map(|line| {
                if let Some(dot_pos) = line.find(". ") {
                    let number_part = &line[..dot_pos];
                    if number_part.chars().all(|c| c.is_ascii_digit()) {
                        Some(line[(dot_pos + 2)..].trim().to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        Block::OrderedList(items)
    } else if first_line.starts_with("> ") {
        let content = lines
            .iter()
            .map(|line| line.trim_start_matches("> ").trim())
            .collect::<Vec<&str>>()
            .join(" ");
        
        Block::Blockquote(content)
    } else {
        Block::Paragraph(lines.join(" "))
    }
}

fn render_blocks_to_html(blocks: Vec<Block>) -> String {
    let mut html_output = String::new();

    for block in blocks {
        match block {
            Block::Header(level, content) => {
                let parsed_content = parse_inline_formatting(&content);
                html_output.push_str(&format!("<h{}>{}</h{}>\n", level, parsed_content, level));
            }
            Block::Paragraph(content) => {
                // let paragraph_content = content.replace('\n', " ");
                let parsed_content = parse_inline_formatting(&content);
                html_output.push_str(&format!("<p>{}</p>\n",parsed_content));
            }
            Block::UnorderedList(items) => {
                html_output.push_str("<ul>\n");
                for item in items {
                    let parsed_item = parse_inline_formatting(&item);
                    html_output.push_str(&format!(" <li>{}</li>\n",parsed_item));
                }
                html_output.push_str("</ul>\n");
            }
            Block::OrderedList(items) => {
                html_output.push_str("<ol>\n");
                for item in items {
                    let parsed_item = parse_inline_formatting(&item);
                    html_output.push_str(&format!("  <li>{}</li>\n", parsed_item));
                }
                html_output.push_str("</ol>\n");
            }
            Block::Blockquote(content) => {
                let parsed_content = parse_inline_formatting(&content);
                html_output.push_str(&format!("<blockquote><p>{}</p></blockquote>\n",parsed_content));
            }
            Block::CodeBlock(content) => {
                html_output.push_str(&format!("<pre><code>{}</code></pre>\n",content));
            }

        }
    }
    html_output
}

fn parse_inline_formatting(text: &str) -> String {
    let mut result = text.to_string();

    loop {
        if let Some(start_bracket) = result.find('[') {
            if let Some(end_bracket_offset) = result[start_bracket..].find(']') {
                let end_bracket = start_bracket + end_bracket_offset;

                if result.chars().nth(end_bracket + 1) == Some('(') {
                    if let Some(end_paren_offset) = result[end_bracket + 2..].find(')') {
                        
                        let end_parenthesis = end_bracket + 2 + end_paren_offset;

                        let link_text = &result[start_bracket + 1..end_bracket];
                        let url = &result[end_bracket + 2..end_parenthesis];

                        let parsed_link_text = parse_inline_formatting(link_text);

                        let replacement = format!("<a href=\"{}\">{}</a>", url, parsed_link_text);
                        result.replace_range(start_bracket..=end_parenthesis, &replacement);

                        continue;
                    }
                }
            }
        }
        break;
    }

    while let Some(start) = result.find("***") {
        if let Some(end) = result[start + 3..].find("***") {
            let end = end + start + 3;
            let content = &result[start + 3..end];
            let replacement = format!("<strong><em>{}</em></strong>", content);
            result.replace_range(start..end + 3, &replacement);
        } else {
            break;
        }
    }
    while let Some(start) = result.find("**") {
        if let Some(end) = result[start + 2..].find("**") {
            let end = end + start + 2;
            let content = &result[start + 2..end];
            let replacement = format!("<strong>{}</strong>", content);
            result.replace_range(start..end + 2, &replacement);
        } else {
            break;
        }
    }

    while let Some(start) = result.find("*") {
        if let Some(end) = result[start + 1..].find("*") {
            let end = end + start + 1;
            let content = &result[start + 1..end];
            let replacement = format!("<em>{}</em>", content);
            result.replace_range(start..end + 1, &replacement);
        } else {
            break;
        }
    }
    result
}