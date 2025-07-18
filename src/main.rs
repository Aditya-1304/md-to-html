use std::fs;
use std::env;

#[derive(Debug)]
enum Block <'a>{
    Header(u8, &'a str),
    Paragraph(&'a str),
    UnorderedList(Vec<&'a str>),
    CodeBlock(&'a str),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: md-forge <input.md> <output.html>");
        return;
    }
    let input_filename = &args[1];
    let output_filename = &args[2];

    println!("Reading from {}...", input_filename);
    let markdown_content = fs::read_to_string(input_filename)
        .expect("Error: Could not read the data form the markdown file");

    let blocks = tokenize(&markdown_content);

    let html_content = render_blocks_to_html(blocks);

    println!("Writing to {}...", output_filename);
    fs::write(output_filename, html_content)
        .expect("Error: Could not write to the html file.");

    println!("Conversion Successful!");
}

fn tokenize(markdown: &str) -> Vec<Block> {
    let mut blocks = Vec::new();

    for block_str in markdown.split("\n\n") {
        let trimmed_block = block_str.trim();
        if trimmed_block.is_empty() {
            continue;
        }


        if trimmed_block.starts_with("```") && trimmed_block.ends_with("```") {
            let content = &trimmed_block[3..trimmed_block.len() - 3].trim();
            blocks.push(Block::CodeBlock(content));
        } else if trimmed_block.starts_with("# ") {
            blocks.push(Block::Header(1, &trimmed_block[2..]));
        } else if trimmed_block.starts_with("## ") {
            blocks.push(Block::Header(2, &trimmed_block[3..]));
        } else if trimmed_block.starts_with("- ") {
            let items = trimmed_block.lines()
                .map(|line| line.trim_start_matches("- ").trim())
                .collect();
            blocks.push(Block::UnorderedList(items));
        } else {
            blocks.push(Block::Paragraph(trimmed_block));
        }
    }
    blocks
}

fn render_blocks_to_html(blocks: Vec<Block>) -> String {
    let mut html_output = String::new();

    for block in blocks {
        match block {
            Block::Header(level, content) => {
                let parsed_content = parse_inline_formatting(content);
                html_output.push_str(&format!("<h{}>{}</h{}>\n", level, parsed_content, level));
            }
            Block::Paragraph(content) => {
                let paragraph_content = content.replace('\n', " ");
                let parsed_content = parse_inline_formatting(&paragraph_content);
                html_output.push_str(&format!("<p>{}</p>\n",parsed_content));
            }
            Block::UnorderedList(items) => {
                html_output.push_str("<ul>\n");
                for item in items {
                    let parsed_item = parse_inline_formatting(item);
                    html_output.push_str(&format!(" <li>{}</li>\n",parsed_item));
                }
                html_output.push_str("</ul>\n");
            }
            Block::CodeBlock(content) => {
                html_output.push_str(&format!("<pre><code>{}</code><pre>\n",content));
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
                    if let Some(end_parenthisis_offset) = result[end_bracket..].find(')') {
                        let end_parenthesis = end_bracket + end_parenthisis_offset;

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
            let replacement = format!("<strong><em>{}</em></strong>",content);
            result.replace_range(start..end + 3, &replacement);
        } else {
            break;
        }
    }
    while let Some(start) = result.find("**") {
        if let Some(end) = result[start + 2..].find("**") {
            let end = end + start + 2;
            let content = &result[start + 2..end];
            let replacement = format!("<strong>{}</strong>",content);
            result.replace_range(start..end + 2, &replacement);
        } else {
            break;
        }
    }

    while let Some(start) = result.find("*") {
        if let Some(end) = result[start + 1..].find("*") {
            let end = end + start + 1;
            let content = &result[start + 1..end];
            let replacement = format!("<em>{}</em>",content);
            result.replace_range(start..end + 1, &replacement);
        } else {
            break;
        }
    }
    result
}