use std::fs::File;
use std::io::{self, BufRead, BufReader,};
use std::path::Path;
use clap::{Arg, Command, ArgAction};

fn print_file_contents(filename: &str, number_lines: bool ) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    for (index, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        
        // Number lines if flag is set
        if number_lines {
            print!("{}\t", index + 1);
        }
        
        
        println!("{}", line);
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // Define CLI arguments using clap
    let matches = Command::new("gaspy")
        .version("0.1.0")
        .about("A Rust implementation of the cat command")

        .arg(Arg::new("files")
            .help("Files to read")
            .index(1)
            .num_args(1..))

        .arg(Arg::new("number")
            .short('n')
            .long("number")
            .help("Number all output lines")
            .default_value("false")
            .action(ArgAction::SetTrue))
        .get_matches();

    // Check if number lines flag is set
    let number_lines = matches.contains_id("number");
    

    // If no files are provided, read from stdin
    if !matches.contains_id("files") {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let  line = line?;
            
            if number_lines {
                print!("{}\t", 1);
            }
            
            
            println!("{}", line);
        }
        return Ok(());
    }

    // Process each file provided as an argument
    if let Some(files) = matches.get_many::<String>("files") {
        for filename in files {
            match print_file_contents(filename, number_lines) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error reading file {}: {}", filename, e);
                }
            }
        }
    }

    Ok(())
}