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

    fnSolvePart1(&lines);
    fnSolvePart2(&lines);
}

fn solvePart1(input: &Vec<String>) {
    // Your part 1 solution goes here
}

fn solvePart2(input &Vec <String>) {
    // Your part 2 solutuon goes here
}
