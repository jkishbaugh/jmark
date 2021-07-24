use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;

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
        let mut _p_tag: bool = false;
        let mut _h_one_tag: bool = false;
        let line_contents = line.unwrap();
        let mut first_char: Vec<char> = line_contents.chars().take(1).collect();
        let mut output = String::new();

        match first_char.pop() {
            Some('#') => {
                if _p_tag {
                    _p_tag = false;
                    output.push_str("</p>\n");
                }
                if _h_one_tag {
                    _h_one_tag = false;
                    output.push_str("</h1>\n");
                }

                _h_one_tag = true;
                output.push_str("<h1>");
                output.push_str(&line_contents[2..]);
            }
            _ => {
                if !_p_tag {
                    _p_tag = true;
                    output.push_str("<p>");
                }
                output.push_str(&line_contents);
            }
        }
        if _p_tag {
            _p_tag =false;
            output.push_str("</p>\n");
        }
        if _h_one_tag {
            _h_one_tag = false;
            output.push_str("</h1>\n");
        }
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
