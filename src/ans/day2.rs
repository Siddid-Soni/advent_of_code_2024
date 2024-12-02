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


// time complexity: O(n)
pub fn part2(lst:Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    for row in lst {
        let mut any_ok = false;

        let mut consider = |x: usize| {
            let mut b = row.clone();
            b.remove(x);
            if is_safe(b) {
                any_ok = true;
            }
        };

        /* remove any element and check is_safe, if it is still safe
            -> the element was the problem, we got the solution
            -> or it was already in safe state 
        either way we need to increment in ans */

        consider(0);
        for i in 0..row.len()-1 {
            let diff = row[i+1] - row[i];
            if diff.abs() < 1 || diff.abs() > 3 {
                consider(i);
                consider(i+1);
                break;
            }

            if i+2 < row.len() {
                let diff2 = row[i+2] - row[i+1];
                if (diff>0) != (diff2>0) { //different signs
                    consider(i);
                    consider(i+1);
                    consider(i+2);
                    break;
                }
            }
        }
        if any_ok {
            ans += 1;
        }
    } 
    ans
}