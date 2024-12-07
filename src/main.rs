mod utils;

use utils::puzzle_input::read_file_lines;

fn main() {
    let path = "input.txt";

    // Read the file and store the lines
    let lines = match read_file_lines(path) {
        Ok(lines) => lines,
        Err(err) => {
            eprintln!("Error reading input file: {}", err);
            return;
        }
    };

    let part_1_answer = solve_part_1(&lines);
    println!("Part 1 answer: {}", part_1_answer);

    let part_2_answer = solve_part_2(&lines);
    println!("Part 2 answer: {}", part_2_answer);
}

fn solve_part_1(input: &Vec<String>) -> String {
    // Your part 1 solution goes here
    let mut answer = 0;
    return answer.to_string();
}

fn solve_part_2(input: &Vec<String>) -> String {
    // Your part 2 solutuon goes here
    let mut answer = 0;
    return answer.to_string();
}
