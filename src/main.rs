use std::fs;
use std::env;

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

    let html_content = parse_markdown_to_html(&markdown_content);

    println!("Writing to {}...", output_filename);
    fs::write(output_filename, html_content)
        .expect("Error: Could not write to the html file.");

    println!("Conversion Successful!");
}

fn parse_markdown_to_html(markdown: &str) -> String {
    let mut html_output = String::new();

    for line in markdown.lines() {
        let trimmed_line = line.trim();

        if trimmed_line.starts_with("## ") {
            let content = &trimmed_line[3..];
            let parsed_content = parse_inline_formatting(content);
            html_output.push_str(&format!("<h2>{}</h2>\n", parsed_content));
        } else if trimmed_line.starts_with("# ") {
            let content = &trimmed_line[2..];
            let parsed_content = parse_inline_formatting(content);
            html_output.push_str(&format!("<h1>{}</h1>\n", parsed_content));
        } else if !trimmed_line.is_empty() {
            let parsed_content = parse_inline_formatting(trimmed_line);
            html_output.push_str(&format!("<p>{}</p>\n", parsed_content));
        }
    }
    html_output
}

fn parse_inline_formatting(text: &str) -> String {
    let mut result = text.to_string();

    while let Some(start) = result.find("**") {
        if let Some(end) = result[start + 2..].find("**") {
            let end = end + start + 2;
            let content = &result[start + 2..end];
            let replacement = format!("<strong>{}</strong>\n",content);
            result.replace_range(start..end + 2, &replacement);
        } else {
            break;
        }
    }

    while let Some(start) = result.find("*") {
        if let Some(end) = result[start + 1..].find("*") {
            let end = end + start + 1;
            let content = &result[start + 1..end];
            let replacement = format!("<em>{}</em>\n",content);
            result.replace_range(start..end + 1, &replacement);
        } else {
            break;
        }
    }
    result
}