use regex::Regex;

pub fn part1(input: &str) -> i32{
    Regex::new(r"mul\(\d+,\d+\)").unwrap().find_iter(input)
            .map(|x| x.as_str().split(",").collect::<Vec<_>>())
            .map(|s| s[0][4..].parse::<i32>().unwrap()*s[1][..s[1].len()-1].parse::<i32>().unwrap()).sum()
}

pub fn part2(input: &str) -> i32 {
    let re = Regex::new(r"don't\(\)|do\(\)|mul\(\d+,\d+\)").unwrap();

    let mut ans = 0;
    let mut flag = true;

    for cap in re.find_iter(input) {
        if cap.as_str() == "do()" {
            flag = true;
        } else if cap.as_str() == "don't()" {
            flag = false;
        } else {
            if flag {
                let s = cap.as_str().split(",").collect::<Vec<_>>();
                ans += s[0][4..].parse::<i32>().unwrap()*s[1][..s[1].len()-1].parse::<i32>().unwrap();
            }
        }
    }
    ans
}