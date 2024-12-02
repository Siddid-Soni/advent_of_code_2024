fn is_safe(row: Vec<i32>) -> bool {
    let mut direction = [0,0]; 
    for i in 0..row.len()-1 {
        let diff = row[i] - row[i+1];
        if diff.abs()<1 || diff.abs()>3 {
            return false;
        }
        direction[i%2] = diff;
        if (direction[0] > 0 && direction[1] < 0) || (direction[0] < 0 && direction[1] > 0) {
            return false;
        }
    }
    true
}

pub fn part1(lst: Vec<Vec<i32>>) -> i32 {
    let mut safe = 0;
    for row in lst {
        if is_safe(row) {
            safe += 1;
        }
    }
    safe
}


pub fn part2(lst: Vec<Vec<i32>>) -> i32 {
    let mut n_safe = 0;
    //Loop over all input
    'outer: for row in lst {
        if is_safe(row.clone()) {
            n_safe +=1;
            continue;
        } 

        //Check all removal possibilities
        for i in 0..row.len() {
            let mut temp_numbers = row.clone();
            temp_numbers.remove(i);
            if is_safe(temp_numbers) {
                n_safe += 1;
                continue 'outer;
            }
        }
    }
    n_safe
}