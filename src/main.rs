use clap::{Arg, Command};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn count_lines_in_file(filename: &Path) -> io::Result<usize> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let count = reader.lines().count();
    Ok(count)
}

fn count_lines_in_dir(dir_path: &Path, recursive: bool) -> io::Result<usize> {
    let mut total_lines = 0;

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            match count_lines_in_file(&path) {
                Ok(count) => total_lines += count,
                Err(e) => eprintln!("Error counting lines in {}: {}", path.display(), e),
            }
        } else if path.is_dir() && recursive {
            match count_lines_in_dir(&path, recursive) {
                Ok(count) => total_lines += count,
                Err(e) => eprintln!("Error processing directory {}: {}", path.display(), e),
            }
        }
    }

    Ok(total_lines)
}

fn count_lines(path_str: &str, recursive: bool) -> io::Result<usize> {
    let path = Path::new(path_str);

    if path.is_file() {
        count_lines_in_file(path)
    } else if path.is_dir() {
        count_lines_in_dir(path, recursive)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Path '{}' is neither a file nor a directory", path_str),
        ))
    }
}

fn print_file_contents(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
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
            Arg::new("count")
                .short('c')
                .long("count")
                .help("Count the number of lines")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("recursive")
                .short('r')
                .long("recursive")
                .help("Process directories recursively")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("files")
                .help("Files or directories to process")
                .index(1)
                .num_args(1..),
        )
        .get_matches();

    let recursive = matches.get_flag("recursive");
    let count_mode = matches.get_flag("count");

    if !matches.contains_id("files") {
        eprintln!("Reading from stdin not yet implemented");
        return Ok(());
    }

    // Process each file/directory provided as an argument
    if let Some(paths) = matches.get_many::<String>("files") {
        let mut total_lines = 0;
        let paths_vec: Vec<_> = paths.collect();

        for path_str in &paths_vec {
            if count_mode {
                match count_lines(path_str, recursive) {
                    Ok(count) => {
                        println!("{}: {} lines", path_str, count);
                        total_lines += count;
                    }
                    Err(e) => {
                        eprintln!("Error processing {}: {}", path_str, e);
                    }
                }
            } else {
                // Only print file contents when NOT in count mode
                if Path::new(path_str).is_file() {
                    if let Err(e) = print_file_contents(path_str) {
                        eprintln!("Error reading file {}: {}", path_str, e);
                    }
                } else {
                    eprintln!("Cannot print contents of directory: {}", path_str);
                }
            }
        }

        if count_mode && paths_vec.len() > 1 {
            println!("Total: {} lines", total_lines);
        }
    }

    Ok(())
}
