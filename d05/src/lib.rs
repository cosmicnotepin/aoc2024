use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn parse_input(
    input: &str,
) -> (
    HashMap<usize, Vec<usize>>,
    Vec<Vec<usize>>,
) {
    let mut smallers = HashMap::new();
    let (rules, updates) = input.split("\n\n").collect_tuple().unwrap();
    for line in rules.lines() {
        let (a, b) = line
            .split('|')
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple().unwrap();
            smallers.entry(b).or_insert(vec![]).push(a);
    }

    let updates = updates
        .lines()
        .map(|l| l.split(',').map(|s| s.parse::<usize>().unwrap()).collect())
        .collect();
    (smallers, updates)
}

fn test_update(
    update: &Vec<usize>,
    smallers: &HashMap<usize, Vec<usize>>,
) -> bool {
    for (i, e) in update.iter().enumerate() {
        for right in &update[i + 1..update.len()] {
            if let Some(smllrs) = smallers.get(e) {
                if smllrs.contains(right) {
                    return false;
                }
            }
        }
    }
    return true;
}

fn fix_update(
    update: &Vec<usize>,
    smallers: &HashMap<usize, Vec<usize>>,
) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    'outer: for ie in &update[0..] {
        for (i, e) in res.iter().enumerate() {
            if let Some(v) = smallers.get(e) {
                if v.contains(ie) {
                    res.insert(i, *ie);
                    continue 'outer;
                }
            }
        }
        res.push(*ie);
    }
    return res;
}

fn part1(input: String) -> i32 {
    let mut res = 0;
    let (smallers, updates) = parse_input(&input);
    for update in updates {
        if test_update(&update, &smallers) {
            res += update[update.len() / 2] as i32;
        }
    }
    return res;
}

fn part2(input: String) -> i32 {
    let mut res = 0;
    let (smallers, updates) = parse_input(&input);
    for update in updates {
        if !test_update(&update, &smallers) {
            res += fix_update(&update, &smallers)[update.len() / 2] as i32;
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
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            .to_string();
        assert_eq!(143, part1(input));
    }

    #[test]
    fn p2_1() {
        let input = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            .to_string();
        assert_eq!(123, part2(input));
    }
}
