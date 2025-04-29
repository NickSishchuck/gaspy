use clap::{Arg, ArgAction, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn count_lines(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut count = 0;
    for _ in reader.lines() {
        count += 1;
    }

    println!("{} lines", count);

    Ok(())
}

fn print_file_contents(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    for (index, line_result) in reader.lines().enumerate() {
        let line = line_result?;

        println!("{}", line);
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // Define CLI arguments using clap
    let matches = Command::new("gaspy")
        .version("0.1.0")
        .about("Gaspy helper tool")
        .arg(
            Arg::new("files")
                .help("Files to read")
                .index(1)
                .num_args(1..),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .help("Count the number of lines")
                .default_value("false")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    // If no files are provided, read from stdin
    if !matches.contains_id("files") {
        let stdin = io::stdin();
        return Ok(());
    }

    // Process each file provided as an argument
    if let Some(files) = matches.get_many::<String>("files") {
        for filename in files {
            match print_file_contents(filename) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error reading file {}: {}", filename, e);
                }
            }
        }
    }

    Ok(())
}
