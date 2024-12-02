# Advent of Code 2024

Welcome to my **Advent of Code 2024** solutions repository! This repo contains my daily coding solutions to the Advent of Code challenges, written in **Rust**. The aim is to solve each problem efficiently while keeping the code clean and readable.

---

## 🗂️ Repository Structure

The project is organized as follows:

```
AdventOfCode2024/
├── src/
│   ├── main.rs        # Entry point for running solutions
│   ├── utils/         # Utility functions for input parsing and other helpers
│   │   └── puzzle_input.rs # Functions to handle reading and parsing inputs
├── tests/             # Integration tests for the solutions
│   └── test_puzzle_input.rs # Tests for utility functions
├── input.txt          # Input file for the current day's challenge
├── scripts/           # Useful scripts for automation (e.g., downloading input)
├── Cargo.toml         # Rust project configuration
├── README.md          # Project overview and details (this file)
└── .devcontainer/     # Development container configuration (optional)
```

Each days problem and solution can be found on a distinct branch named `day-XX`.

---

## 🔍 Files and Directories

### 1. **src/**
- **`main.rs`**: The entry point of the application. Each day's solution is written here.
- **`utils/`**: Contains reusable utility functions, such as reading and parsing input files.

### 2. **tests/**
- Contains integration tests to ensure the correctness of parsing utilities and solutions.

### 3. **input.txt**
- The input file for the current day's challenge. Since each day has a separate branch, only one `input.txt` file is needed per branch.

### 4. **scripts/**
- Scripts for automating tasks, such as downloading inputs or setting up the environment.

### 5. **Cargo.toml**
- The project’s configuration file, defining dependencies and project metadata.

### 6. **.devcontainer/** *(optional)*
- Configuration for the development container to ensure a consistent coding environment.

---

## 🚀 Running a Solution

1. **Set Up the Development Container**:
   - Open the repository in VS Code.
   - Ensure you have the Remote - Containers extension installed.
   - Open the Command Palette (`Ctrl+Shift+P`), select `Remote-Containers: Reopen in Container`, and wait for the container to build.

2. **Place the Input File**:
   - Place the input file for the current day's challenge as `input.txt` in the root directory.

3. **Run the Solution**:
   - Inside the container, you can run the solution with:
     ```bash
     cargo run
     ```

4. **Use the Scripts**:
   - The `scripts/` directory contains automation scripts for tasks like downloading inputs. For example:
     ```bash
     ./scripts/download_input.sh
     ```
   - Ensure these scripts are executable by running `chmod +x ./scripts/*` if needed.

---

## 🧪 Testing

To test utility functions and logic:

```bash
cargo test
```

---

## 🎯 Goals

- **Daily Problem Solving**: Commit solutions for all 25 days of the Advent of Code.
- **Efficient Code**: Focus on performance and clarity.
- **Polished Project**: Keep the repository well-structured and professional.

---

Happy coding! 🎄✨

