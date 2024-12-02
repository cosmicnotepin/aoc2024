use std::error::Error;
use std::fs;
use std::iter::zip;

fn safe(report: &str) -> bool {
    let levels: Vec<i32> = report
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let increasing = levels[1] > levels[0];
    for (n1, n2) in zip(&levels[..(levels.len() - 1)], &levels[1..]) {
        let diff = (n2 - n1).abs();
        if (n2 > n1) != increasing || (diff > 3 || diff == 0) {
            return false;
        }
    }
    return true;
}

fn safe_damp(report: &str) -> bool {
    let levels: Vec<i32> = report
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    for i in 0..levels.len() + 1 {
        let mut dampened = String::new();
        for (j, n) in levels.iter().enumerate() {
            if i != j {
                dampened.push_str(&(n.to_string()));
                dampened.push(' ');
            }
        }
        if safe(dampened.trim_end()) {
            return true;
        }
    }
    return false;
}

fn part1(input: String) -> i32 {
    let mut res = 0;
    for l in input.lines() {
        if safe(l) {
            res += 1;
        }
    }
    return res;
}

fn part2(input: String) -> i32 {
    let mut res = 0;
    for l in input.lines() {
        if safe_damp(l) {
            res += 1;
        }
    }
    return res;
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let input1 = fs::read_to_string("input1")?;
    println!("part 1: {}", part1(input1));
    let input2 = fs::read_to_string("input1")?;
    println!("part 2: {}", part2(input2));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_1() {
        let input = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            .to_string();
        let safeness = [true, false, false, false, false, true];
        for (l, s) in zip(input.lines(), safeness) {
            assert_eq!(s, safe(l));
        }
        assert_eq!(2, part1(input));
    }

    #[test]
    fn p2_1() {
        let input = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            .to_string();
        let safeness = [true, false, false, true, true, true];
        for (l, s) in zip(input.lines(), safeness) {
            assert_eq!(s, safe_damp(l));
        }
        assert_eq!(4, part2(input));
    }
}
