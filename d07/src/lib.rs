use std::error::Error;
use std::fs;
use std::time::Instant;

fn part1(input: String) -> usize {
    let mut res = 0;
    for line in input.lines() {
        let (val, numbers) = line.split_once(": ").unwrap();
        let val: usize = val.parse().unwrap();
        let numbers: Vec<usize> = numbers.split(' ').map(|n| n.parse().unwrap()).collect();
        let mut calc_vals = vec![numbers[0]];
        let operators: Vec<fn(usize, usize) -> usize> = vec![|x, y| x + y, |x, y| x * y];
        for n in &numbers[1..] {
            let mut new_calc_vals: Vec<usize> = Vec::new();
            for cv in calc_vals {
                for op in &operators {
                    let inter = op(cv, *n);
                    if inter <= val {
                        new_calc_vals.push(inter);
                    }
                }
            }
            calc_vals = new_calc_vals;
        }
        if calc_vals.contains(&val) {
            res += val;
        }
    }
    return res;
}

fn part2(input: String) -> usize {
    let mut res = 0;
    for line in input.lines() {
        let (val, numbers) = line.split_once(": ").unwrap();
        let val: usize = val.parse().unwrap();
        let numbers: Vec<usize> = numbers.split(' ').map(|n| n.parse().unwrap()).collect();
        let mut calc_vals = vec![numbers[0]];
        let operators: Vec<fn(usize, usize) -> usize> = vec![|x, y| x + y, |x, y| x * y, |x, y| {
            x * 10usize.pow(x.ilog10() + 1) + y
        }];
        for n in &numbers[1..] {
            let mut new_calc_vals: Vec<usize> = Vec::new();
            for cv in calc_vals {
                for op in &operators {
                    let inter = op(cv, *n);
                    if inter <= val {
                        new_calc_vals.push(inter);
                    }
                }
            }
            calc_vals = new_calc_vals;
        }
        if calc_vals.contains(&val) {
            res += val;
        }
    }
    return res;
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let before1 = Instant::now();
    let input1 = fs::read_to_string("input1")?;
    let p1 = part1(input1);
    println!("part 1: {} in {:.2?}", p1, before1.elapsed());
    let before2 = Instant::now();
    let input2 = fs::read_to_string("input1")?;
    let p2 = part2(input2);
    println!("part 2: {} in {:.2?}", p2, before2.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_1() {
        let input = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            .to_string();
        assert_eq!(3749, part1(input));
    }

    #[test]
    fn p2_1() {
        let input = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            .to_string();
        assert_eq!(11387, part2(input));
    }
}
