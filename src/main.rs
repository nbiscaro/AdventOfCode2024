mod utils;

use utils::puzzle_input::read_file_lines;

fn main() {
    let path = "input.txt";

    // Read the file and store the lines
    let _lines = match read_file_lines(path) {
        Ok(lines) => lines,
        Err(err) => {
            eprintln!("Error reading input file: {}", err);
            return;
        }
    };
}
