use std::fs;
use std::io::{self, BufRead};

/// Reads the contents of a file line by line into a `Vec<String>`.
///
/// # Arguments
/// * `path` - A string slice that holds the path to the input file.
///
/// # Returns
/// * `Ok(Vec<String>)` containing the lines of the file if successful.
/// * `Err(io::Error)` if the file cannot be read.
pub fn read_file_lines(path: &str) -> io::Result<Vec<String>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().collect::<Result<Vec<_>, _>>()?)
}
