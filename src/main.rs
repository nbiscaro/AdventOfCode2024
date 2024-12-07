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

    solve_part_1(&lines);
    solve_part_2(&lines);
}

fn solve_part_1(input: &Vec<String>) {
    // Your solution goes here

    // Given two columns of numbers sum the minimum pairwise we need to:
    // Pair numbers of the same rank
    // Find the distance betwen each pair

    let mut list_1: Vec<i32> = Vec::new();
    let mut list_2: Vec<i32> = Vec::new();

    for line in input {
        let mut words = line.split_whitespace();
        let word_1 = words.next().unwrap_or_default();
        let word_2 = words.next().unwrap_or_default();

        list_1.push(word_1.parse().expect("Failed to parse word1"));
        list_2.push(word_2.parse().expect("Failed to parse word2"))
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

fn solve_part_2(input: &Vec<String>) {
    // Create map for right list of the form {number : frequency}
    // For each number in the left list multiply by frequency for that number in the right list

    use std::collections::HashMap;

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_map: HashMap<i32, i32> = HashMap::new();

    for line in input {
        let mut row = line.split_whitespace();

        let word_1 = row.next().unwrap_or_default();
        let word_2 = row.next().unwrap_or_default();

        let left_num: i32 = word_1.parse().expect("Failed to parse word1");
        let right_num: i32 = word_2.parse().expect("Failed to parse word2");

        *right_map.entry(right_num).or_insert(0) += 1;
        left_list.push(left_num);
    }

    let mut similarity_score: i32 = 0;
    for left_num in left_list {
        similarity_score += left_num * right_map.get(&left_num).unwrap_or(&0);
    }

    println!("Part 2 answer: {}", similarity_score);
}
