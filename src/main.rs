mod ans;
use ans::{day1, day2};
use advent_of_code::get_vectors;
use code_timing_macros::time_snippet;
use advent_of_code::get_matrix;

#[allow(unused_assignments)]
fn main() {
    println!("Day 1");
    println!("===Part 1===");
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    (vec1, vec2) = get_vectors("/home/siddid/advent_of_code/src/input/day1.txt");
    let result = time_snippet!(day1::part1(vec1, vec2));

    println!("Result: {}", result);

    println!("===Part 2===");
    (vec1, vec2) = get_vectors("/home/siddid/advent_of_code/src/input/day1.txt");
    let result = time_snippet!(day1::part2(vec1, vec2));

    println!("Result: {}\n", result);

    println!("Day 2");
    println!("===Part 1===");
    let matrix = get_matrix("/home/siddid/advent_of_code/src/input/day2.txt");
    let result = time_snippet!(day2::part1(matrix));
    println!("Result: {}", result);
    println!("===Part 2===");
    let matrix = get_matrix("/home/siddid/advent_of_code/src/input/day2.txt");
    let result = time_snippet!(day2::part2(matrix));
    println!("Result: {}", result);
}