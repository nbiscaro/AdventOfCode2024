#[cfg(test)]
mod tests {
    use advent_of_code_2024::utils::puzzle_input::read_file_lines;

    #[test]
    fn test_read_file_lines() {
        let path = "tests/test_data/test_input.txt";

        // Ensure the file exists for testing
        assert!(
            std::path::Path::new(path).exists(),
            "Test file does not exist: {}",
            path
        );

        // Test reading the file line by line
        let lines = read_file_lines(path).unwrap();
        assert_eq!(lines.len(), 3, "Expected 3 lines, found {}", lines.len());
    }
}
