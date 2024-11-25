use std::env;
use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("\nFormat as : cargo run find_text replace_text file_path\n");
        std::process::exit(1);
    }

    let find_text = &args[1];
    let replace_text = &args[2];
    let file_path = &args[3];

    if !fs::metadata(file_path).is_ok() {
        eprintln!("\nError: Filename '{}' does not find.\n", file_path);
        std::process::exit(1);
    }

    let file = fs::File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut line_number = 1;
    let mut replaced_lines = Vec::new();
    let mut found_text = false;

    for line in reader.lines() {
        let line = line.unwrap();
        let new_line = line.replace(find_text, replace_text);

        if new_line != line {
            println!("\nReplaced text on line {}: {}\n", line_number, new_line);
            found_text = true;
        }

        replaced_lines.push(new_line);
        line_number += 1;
    }

    if !found_text {
        println!("\nFind text '{}' does not find in {}\n", find_text, file_path);
    } else {
        fs::write(file_path, replaced_lines.join("\n"))
            .expect("Able to write to the file");
    }
}