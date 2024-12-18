use itertools::Itertools;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::time::Instant;

fn part1(input: String) -> String {
    let re = Regex::new(r"\d+").unwrap();
    let (register_s, program_s) = input.split_once("\n\n").unwrap();
    let (a, b, c) = re
        .find_iter(register_s)
        .map(|n| n.as_str().parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    let program: Vec<usize> = re
        .find_iter(program_s)
        .map(|n| n.as_str().parse::<usize>().unwrap())
        .collect();
    let mut ip: usize = 0;
    let mut combo = [0, 1, 2, 3, a, b, c, 7];
    let mut out: String = String::new();
    while ip < program.len() {
        let opc = program[ip];
        let opr = program[ip + 1];
        let mut jumped = false;
        match opc {
            0 => combo[4] /= 2usize.pow(combo[opr] as u32),
            1 => combo[5] ^= opr,
            2 => combo[5] = combo[opr] % 8,
            3 => {
                if combo[4] != 0 {
                    ip = opr;
                    jumped = true;
                }
            }

            4 => combo[5] ^= combo[6],
            5 => out += &(((combo[opr] % 8).to_string()) + ","),
            6 => combo[5] = combo[4] / 2usize.pow(combo[opr] as u32),
            7 => combo[6] = combo[4] / 2usize.pow(combo[opr] as u32),
            _ => println!("other"),
        }
        if !jumped {
            ip += 2;
        }
    }
    out.pop();
    return out;
}

fn part2(input: String) -> usize {
    let re = Regex::new(r"\d+").unwrap();
    let (register_s, program_s) = input.split_once("\n\n").unwrap();
    let (_, b, c) = re
        .find_iter(register_s)
        .map(|n| n.as_str().parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    let program: Vec<usize> = re
        .find_iter(program_s)
        .map(|n| n.as_str().parse::<usize>().unwrap())
        .collect();
    let mut a = 0;
    'outer: loop {
        let mut out_i = 0;
        let mut ip: usize = 0;
        let mut combo = [0, 1, 2, 3, a, b, c, 7];
        while ip < program.len() {
            let opc = program[ip];
            let opr = program[ip + 1];
            let mut jumped = false;
            match opc {
                0 => combo[4] /= 2usize.pow(combo[opr] as u32),
                1 => combo[5] ^= opr,
                2 => combo[5] = combo[opr] % 8,
                3 => {
                    if combo[4] != 0 {
                        ip = opr;
                        jumped = true;
                    }
                }

                4 => combo[5] ^= combo[6],
                5 => {
                    if (combo[opr] % 8) != program[out_i] {
                        a += 1;
                        continue 'outer;
                    }
                    out_i += 1;
                }
                6 => combo[5] = combo[4] / 2usize.pow(combo[opr] as u32),
                7 => combo[6] = combo[4] / 2usize.pow(combo[opr] as u32),
                _ => println!("other"),
            }
            if !jumped {
                ip += 2;
            }
        }
        if out_i == program.len() {
            break;
        }
        a += 1;
    }
    return a;
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
    fn p2_1() {
        let input = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
            .to_string();
        assert_eq!(117440, part2(input));
    }

    #[test]
    fn p1_1() {
        let input = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
            .to_string();
        assert_eq!("4,6,3,5,6,3,5,2,1,0", part1(input));
    }

    #[test]
    fn p1_2() {
        let input = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
            .to_string();
        assert_eq!("4,2,5,6,7,7,7,7,3,1,0", part1(input));
    }

    #[test]
    fn p1_3() {
        let input = "\
Register A: 0
Register B: 0
Register C: 9

Program: 2,6"
            .to_string();
        assert_eq!("", part1(input));
    }

    #[test]
    fn p1_4() {
        let input = "\
Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4"
            .to_string();
        assert_eq!("0,1,2", part1(input));
    }

    #[test]
    fn p1_5() {
        let input = "\
Register A: 0
Register B: 29
Register C: 0

Program: 1,7"
            .to_string();
        assert_eq!("", part1(input));
    }

    #[test]
    fn p1_6() {
        let input = "\
Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0"
            .to_string();
        assert_eq!("", part1(input));
    }
}
