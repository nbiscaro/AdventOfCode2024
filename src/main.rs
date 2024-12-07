mod utils;

use utils::puzzle_input::read_file_lines;

fn main() {
    let path = "input.txt";

    // Read the file and store the lines
    let lines: Vec<String> = match read_file_lines(path) {
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
    // Your solution goes here

    // Given two columns of numbers sum the minimum pairwise we need to:
    // Pair numbers of the same rank
    // Find the distance betwen each pair

    let mut list_1: Vec<i32> = Vec::new();
    let mut list_2: Vec<i32> = Vec::new();

    for line in input {
        let words: Vec<&str> = line.split_whitespace().collect();

        if let [word_1, word_2] = &words[..] {
            list_1.push((*word_1).parse::<i32>().expect("Failed to parse word1"));
            list_2.push((*word_2).parse::<i32>().expect("Failed to parse word2"));
        } else {
            eprintln!("Unexpected input format");
        }
    }

    list_1.sort();
    list_2.sort();

    let edit_distance: i32 = list_1
        .iter()
        .zip(list_2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Part 1 answer: {}", edit_distance);
}

fn solvePart2(input &Vec <String>) {
    // Your part 2 solutuon goes here
}
