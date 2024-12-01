use std::collections::HashMap;

pub fn part1(mut v1: Vec<i32>, mut v2: Vec<i32>) -> u32 {
    v1.sort_unstable();
    v2.sort_unstable();

    v1.iter().zip(v2.iter()).map(|(x,y)| (x).abs_diff(*y)).sum()
}

pub fn part2(v1: Vec<i32>, v2: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for x in v2.iter() {
        map.entry(x).and_modify(|c| *c += 1).or_insert(1);
    }
    v1.iter().map(|x| x * map.get(x).unwrap_or(&0)).sum()
}