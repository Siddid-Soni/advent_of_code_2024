pub mod ans;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

pub fn get_vectors<P>(filename: P) -> (Vec<i32>, Vec<i32>)
where P: AsRef<Path>, {
    if let Ok(lines) = read_lines(filename) {
        lines.map(|line| {
            let res = line.unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            (res[0], res[1])
        }).unzip()
    } else {
        (vec![], vec![])
    }
}

pub fn get_matrix<P>(filename: P)->Vec<Vec<i32>> where P: AsRef<Path>, {
    if let Ok(lines) = read_lines(filename) {
        lines.map(|line| {
            line.unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
        }).collect()
    } else {
        vec![]
    }
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ans::{day1, day2};

    #[test]
    #[allow(unused_assignments)]
    fn day1_part1() {
        let mut vec1 = Vec::new();
        let mut vec2 = Vec::new();
        (vec1, vec2) = get_vectors("/home/siddid/advent_of_code/src/input/test1.txt");
        assert_eq!(day1::part1(vec1, vec2),11);
    }

    #[test]
    #[allow(unused_assignments)]
    fn day1_part2() {
        let mut vec1 = Vec::new();
        let mut vec2 = Vec::new();
        (vec1, vec2) = get_vectors("/home/siddid/advent_of_code/src/input/test1.txt");
        assert_eq!(day1::part2(vec1, vec2), 31);
    }

    #[test]
    fn day2_part1() {
        let matrix = get_matrix("/home/siddid/advent_of_code/src/input/test2.txt");
        assert_eq!(day2::part1(matrix), 2);
    }

    #[test]
    fn day2_part2() {
        let matrix = get_matrix("/home/siddid/advent_of_code/src/input/test2.txt");
        assert_eq!(day2::part2(matrix), 4);
    }
}
