use itertools::Itertools;
use std::cmp::max;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::time::Instant;

fn part1(input: String) -> isize {
    let mut res = 0;
    for mut sec in input.lines().map(|s| s.parse::<isize>().unwrap()) {
        for _ in 0..2000 {
            sec = (sec ^ (sec * 64)) % 16777216;
            sec = (sec ^ (sec / 32)) % 16777216;
            sec = (sec ^ (sec * 2048)) % 16777216;
        }
        res += sec;
    }
    return res;
}

#[allow(dead_code)]
fn part2(input: String) -> isize {
    let mut sequences: Vec<Vec<(isize, isize)>> = Vec::new();
    for mut sec in input.lines().map(|s| s.parse::<isize>().unwrap()) {
        let mut sequence: Vec<(isize, isize)> = Vec::new();
        for _ in 0..2000 {
            let o_sec = sec % 10;
            sec = (sec ^ (sec * 64)) % 16777216;
            sec = (sec ^ (sec / 32)) % 16777216;
            sec = (sec ^ (sec * 2048)) % 16777216;
            sequence.push((sec % 10, (sec % 10) - o_sec));
        }
        sequences.push(sequence);
    }
    let mut best = 0;
    for i in -9..9 {
        for j in -9..9 {
            for k in -9..9 {
                for l in -9..9 {
                    let mut cur = 0;
                    for sequence in &sequences {
                        for n in 0..(sequence.len() - 3) as usize {
                            if sequence[n].1 == i
                                && sequence[n + 1].1 == j
                                && sequence[n + 2].1 == k
                                && sequence[n + 3].1 == l
                            {
                                cur += sequence[n + 3].0;
                                break;
                            }
                        }
                    }
                    best = max(best, cur);
                }
            }
        }
    }
    return best;
}

fn part2_smarter(input: String) -> isize {
    let mut res: HashMap<(isize, isize, isize, isize), isize> = HashMap::new();
    for mut sec in input.lines().map(|s| s.parse::<isize>().unwrap()) {
        let mut sequence: Vec<(isize, isize)> = Vec::new();
        for _ in 0..2000 {
            let o_sec = sec % 10;
            sec = (sec ^ (sec * 64)) % 16777216;
            sec = (sec ^ (sec / 32)) % 16777216;
            sec = (sec ^ (sec * 2048)) % 16777216;
            sequence.push((sec % 10, (sec % 10) - o_sec));
        }
        let mut seen = HashSet::new();
        for (a, b, c, d) in sequence.iter().tuple_windows() {
            if seen.insert((a.1, b.1, c.1, d.1)) {
                *res.entry((a.1, b.1, c.1, d.1)).or_default() += d.0
            }
        }
    }
    return *res.values().max().unwrap();
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let before1 = Instant::now();
    let input1 = fs::read_to_string("input1")?;
    let p1 = part1(input1);
    println!("part 1: {} in {:.2?}", p1, before1.elapsed());
    let before2 = Instant::now();
    let input2 = fs::read_to_string("input1")?;
    let p2 = part2_smarter(input2);
    println!("part 2: {} in {:.2?}", p2, before2.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_1() {
        let input = "\
1
10
100
2024"
            .to_string();
        assert_eq!(37327623, part1(input));
    }

    #[test]
    fn p2_1() {
        let input = "\
1
2
3
2024"
            .to_string();
        assert_eq!(23, part2_smarter(input));
    }
}
