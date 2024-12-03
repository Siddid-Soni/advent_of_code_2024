# Advent of Code 2024

This repository contains solutions for the Advent of Code 2024 challenges implemented in Rust.

## Usage

To run the solutions, ensure you have Rust installed on your system. You can install Rust from [official Rust website](https://www.rust-lang.org/).

### Running the Code

1. Clone the repository:
   ```sh
   git clone https://github.com/Siddid-Soni/advent_of_code_2024.git
   cd advent_of_code_2024
   ```

2. Ensure you have the input files for each day in the `src/input` directory.

3. Run the main program:
   ```sh
   cargo run
   ```

The main program will execute the solutions for each day and display the results in the console.

### Running Tests

To run the tests, use the following command:
```sh
cargo test
```

### Rust Modules Structure

The code is organized into several modules, each corresponding to a day's challenge. Here is the structure of the modules:

- `src/ans/mod.rs`: This file includes the modules for each day.
- `src/ans/dayX.rs`: Contains the solution implementations for day X.
- `src/input/`: Directory containing input files for each day.

### Folder Structure

Below is the folder structure of the repository:

```
advent_of_code_2024/
├── src/
│   ├── ans/
│   │   ├── mod.rs
│   │   ├── day1.rs
│   │   ├── day2.rs
│   │   ├── day3.rs
│   ├── input/
│   │   ├── day1.txt
│   │   ├── day2.txt
│   │   ├── day3.txt
│   ├── main.rs
├── Cargo.toml
```

### Example Code Structure

Below is a brief overview of the code structure:

- `src/main.rs`: The main entry point of the program, which runs the solutions for each day.
- `src/ans/dayX.rs`: Contains the solution implementations for day X.
- `src/input/`: Directory containing input files for each day.

### Sample Output

When you run the program, you will see outputs similar to the following:

```
Day 1
===Part 1===
src/main.rs:14 took 412.623µs.
Result: <result>
===Part 2===
src/main.rs:19 took 485.99µs.
Result: <result>

Day 2
===Part 1===
src/main.rs:25 took 97.386µs.
Result: <result>
===Part 2===
src/main.rs:29 took 364.908µs.
Result: <result>

Day 3
===Part 1===
src/main.rs:35 took 2.329485ms.
Result: <result>
===Part 2===
src/main.rs:39 took 3.818173ms.
Result: <result>
```

## Contributing

Feel free to open issues or submit pull requests if you find any bugs or want to add enhancements.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.