use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;
use crate::TagType::{P, H1};

enum TagType {
    H1,
    P
}

fn get_closing_tag(_tag_to_close: TagType) -> String {
    match _tag_to_close {
        H1 => "</h1>\n",
        P => "</p>\n",
    }.to_string()
}
fn get_title() -> String {
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));
    the_title.push_str(" (v");
    the_title.push_str(env!("CARGO_PKG_VERSION"));
    the_title.push_str(") ");
    the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));

    return the_title;
}

fn parse_markdown_file(_filename: &str) {
    print_short_banner();
    println!("[Info] Starting Parser");

    let file_to_parse = Path::new(_filename);
    let file = File::open(&file_to_parse).expect("[Error] Failed to open file!");
    let mut tokens: Vec<String> = Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let mut _tag_type: TagType= P;

        let line_contents = line.unwrap();
        let mut first_char: Vec<char> = line_contents.chars().take(1).collect();
        let mut output = String::new();

        match first_char.pop() {
            Some('#') => {
                _tag_type= H1;
                output.push_str("<h1>");
                output.push_str(&line_contents[2..]);
            }
            _ => {
                output.push_str("<p>");
                output.push_str(&line_contents);
            }
        }
        output.push_str(&get_closing_tag(_tag_type));
        if output != "<p></p>\n"{
            tokens.push(output);
        }
    }
    let mut html_filename = String::from(&_filename[.._filename.len()-3]);
    html_filename.push_str(".html");
    let mut outfile = File::create(&html_filename).expect("[Error] Could not create output file.");
    for token in &tokens {
        outfile.write_all(token.as_bytes()).expect(" [Error] Could not write to output file.");
    }
    println!("[ INFO ] Finished parsing.");
}

fn print_short_banner() {
    println!("{}", get_title())
}

fn print_long_banner() {
    let author = env!("CARGO_PKG_AUTHORS");
    print_short_banner();
    println!("Written by: {}\nUsage: jmark <somefile>.md", author)
}

fn usage() {
    print_long_banner();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("[Error] Invalid number of arguments.");
            usage();
        }
    }
}
