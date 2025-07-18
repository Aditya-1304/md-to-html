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
            html_output.push_str(&format!("<h2>{}</h2>\n", content));
        } else if trimmed_line.starts_with("# ") {
            let content = &trimmed_line[2..];
            html_output.push_str(&format!("<h1>{}</h1>\n", content));
        } else if !trimmed_line.is_empty() {
            html_output.push_str(&format!("<p>{}</p>\n", trimmed_line));
        }
    }
    html_output
}