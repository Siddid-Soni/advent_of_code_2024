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

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ans::day1::{part1, part2};

    #[test]
    #[allow(unused_assignments)]
    fn day1_part1() {
        let mut vec1 = Vec::new();
        let mut vec2 = Vec::new();
        (vec1, vec2) = get_vectors("/home/siddid/advent_of_code/src/input/test.txt");
        assert_eq!(part1(vec1, vec2),11);
    }

    #[test]
    #[allow(unused_assignments)]
    fn day1_part2() {
        let mut vec1 = Vec::new();
        let mut vec2 = Vec::new();
        (vec1, vec2) = get_vectors("/home/siddid/advent_of_code/src/input/test.txt");
        assert_eq!(part2(vec1, vec2), 31);
    }
}
