use std::env;
use std::io::{self, BufRead};
use std::process;

fn usage() {
    println!("rnl is newline convertor.

Usage:

    $ # Convert to LF
    $ cat file.txt | rnl lf

Supported newlines:

    lf ... UNIX and Mac's newline
    cr ... Old Mac's newline
    crlf ... Windows's newline
");
    process::exit(0);
}

pub fn conv_line(line: &String, nl: &str) -> String {
    return format!("{}{}", line, nl);
}

fn run_app(args: &Vec<String>) {
    if args.len() < 2 {
        usage();
    }

    let stdin = io::stdin();
    let to: &String = &args[1].to_lowercase();
    let mut nl: &str = "\n";

    match to.as_str() {
        "lf" => nl = "\n",
        "cr" => nl = "\r",
        "crlf" => nl = "\r\n",
        _ => usage(),
    }

    for line in stdin.lock().lines() {
        let s: String = conv_line(&line.unwrap(), nl);
        print!("{}", s);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    run_app(&args);
}

#[cfg(test)]
mod tests {
    use crate::conv_line;

    #[test]
    fn test_conv_line() {
        assert_eq!(conv_line(&String::from(""), "\n"), "\n");
        assert_eq!(conv_line(&String::from(""), "\r"), "\r");
        assert_eq!(conv_line(&String::from(""), "\r\n"), "\r\n");
        assert_eq!(conv_line(&String::from("abc"), "\n"), "abc\n");
    }
}
