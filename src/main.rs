use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader,};
use std::path::Path;

fn print_file_contents(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // If no arguments are provided, read from stdin
    if args.len() == 1 {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            println!("{}", line?);
        }
        return Ok(());
    }

    // Process each file provided as an argument
    for filename in &args[1..] {
        match print_file_contents(filename) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error reading file {}: {}", filename, e);
            }
        }
    }

    Ok(())
}