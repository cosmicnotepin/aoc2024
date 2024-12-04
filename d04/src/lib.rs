use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn rows(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>()
}

fn cols(input: &str) -> Vec<String> {
    let mut res = Vec::new();
    input.lines().for_each(|_| res.push(String::new()));
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            res[i].push(c);
        }
    }
    res
}

fn diag(input: &str) -> Vec<String> {
    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let starting_indices = (0..matrix.len())
        .map(|x| (x, 0))
        .chain((1..matrix[0].len()).map(|x| (0, x)));

    let mut res = Vec::new();
    for (mut i, mut j) in starting_indices {
        let mut line = String::new();
        loop {
            if let Some(l) = matrix.get(i) {
                if let Some(c) = l.get(j) {
                    line.push(c.clone());
                } else {
                    break;
                }
            } else {
                break;
            }
            i += 1;
            j += 1;
        }
        res.push(line);
    }

    let starting_indices = (0..matrix.len())
        .map(|x| (x, matrix[0].len() - 1))
        .chain((0..matrix[0].len() - 1).map(|x| (0, x)));

    for (mut i, mut j) in starting_indices {
        let mut line = String::new();
        loop {
            if let Some(l) = matrix.get(i) {
                if let Some(c) = l.get(j) {
                    line.push(c.clone());
                } else {
                    break;
                }
            } else {
                break;
            }
            if j == 0 {
                break;
            }
            i += 1;
            j -= 1;
        }
        res.push(line);
    }
    res
}

fn part1b(input: String) -> i32 {
    let re = Regex::new(r"XMAS").unwrap();

    rows(&input)
        .iter()
        .chain(&cols(&input))
        .chain(&diag(&input))
        .map(|s| {
            (re.find_iter(s).count() + re.find_iter(&(s.chars().rev().collect::<String>())).count())
                as i32
        })
        .sum()
}

fn part1(input: String) -> i32 {
    let mut map = HashMap::new();
    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            map.insert((i as i32, j as i32), c);
        }
    }
    let mut dirs = Vec::new();
    for i in -1..2 {
        for j in -1..2 {
            if (i == 0) && (j == 0) {
                continue;
            }
            dirs.push((i, j));
        }
    }

    let mas = [('M', 1), ('A', 2), ('S', 3)];
    let mut res = 0;
    for ((i, j), c) in map.iter() {
        if *c != 'X' {
            continue;
        }
        'outer: for (row, col) in &dirs {
            if !map.contains_key(&(i + row * 3, j + col * 3)) {
                continue;
            }
            for (cc, d) in mas {
                if *(map.get(&(i + row * d, j + col * d)).unwrap()) != cc {
                    continue 'outer;
                }
            }
            res += 1
        }
    }
    res
}

fn part2(input: String) -> i32 {
    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let mut res = 0;
    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[0].len() - 1 {
            if matrix[i][j] != 'A' {
                continue;
            }
            let ul = matrix[i - 1][j - 1];
            let lr = matrix[i + 1][j + 1];
            let ur = matrix[i - 1][j + 1];
            let ll = matrix[i + 1][j - 1];
            if ((ul == 'M' && lr == 'S') || (ul == 'S' && lr == 'M'))
                && ((ur == 'M' && ll == 'S') || (ur == 'S' && ll == 'M'))
            {
                res += 1;
            }
        }
    }
    res
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
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .to_string();
        //println!("rows(&input): {:?}", rows(&input));
        //println!("cols(&input): {:?}", cols(&input));
        //println!("diag(&input): {:?}", diag(&input));
        assert_eq!(18, part1(input));
    }

    #[test]
    fn p1_1b() {
        let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .to_string();
        assert_eq!(18, part1b(input));
    }

    #[test]
    fn p2_1() {
        let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .to_string();
        assert_eq!(9, part2(input));
    }
}
