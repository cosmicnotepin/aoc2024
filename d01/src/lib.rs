use std::error::Error;
use std::fs;
use std::iter::zip;

fn part1(input: String) -> i32 {
    let mut l1: Vec<i32> = Vec::new();
    let mut l2: Vec<i32> = Vec::new();
    for line in input.lines() {
        match line.split("   ").collect::<Vec<&str>>()[..] {
            [n1, n2] => {
                l1.push(n1.parse().expect("in aoc we trust"));
                l2.push(n2.parse().expect("in aoc we trust"));
            }
            _ => panic!("unexpected file format"),
        }
    }
    l1.sort();
    l2.sort();
    let mut res = 0;
    for (n1, n2) in zip(l1, l2) {
        res += (n1 - n2).abs();
    }
    return res;
}

fn part2(input: String) -> i32 {
    let mut l1: Vec<i32> = Vec::new();
    let mut l2: Vec<i32> = Vec::new();
    for line in input.lines() {
        match line.split("   ").collect::<Vec<&str>>()[..] {
            [n1, n2] => {
                l1.push(n1.parse().expect("in aoc we trust"));
                l2.push(n2.parse().expect("in aoc we trust"));
            }
            _ => panic!("unexpected file format"),
        }
    }
    let mut res: i32 = 0;
    for n1 in l1 {
        let l2_count = l2.iter().filter(|&&n2| n2 == n1).count();
        res += n1 * <usize as TryInto<i32>>::try_into(l2_count).unwrap()
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
3   4
4   3
2   5
1   3
3   9
3   3"
            .to_string();
        assert_eq!(11, part1(input));
    }

    #[test]
    fn p2_1() {
        let input = "\
3   4
4   3
2   5
1   3
3   9
3   3"
            .to_string();
        assert_eq!(31, part2(input));
    }
}
