use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::time::Instant;

fn part1(input: String) -> usize {
    let mut nodes: HashMap<String, HashSet<String>> = HashMap::new();
    for line in input.lines() {
        let (l, r) = line.split_once('-').unwrap();
        let (l, r) = (l.to_string(), r.to_string());
        nodes.entry(l.clone()).or_default().insert(r.clone());
        nodes.entry(r).or_default().insert(l);
    }
    let mut sets_of_three: Vec<[String; 3]> = Vec::new();
    let mut seen = HashSet::new();
    for (node, conns) in &nodes {
        for (a, b) in conns.iter().tuple_combinations() {
            if seen.contains(a) || seen.contains(b) {
                continue;
            }
            if nodes[a].contains(b) && nodes[b].contains(a) {
                sets_of_three.push([node.clone(), a.clone(), b.clone()]);
            }
        }
        seen.insert(node.clone());
    }
    return sets_of_three
        .iter()
        .filter(|&s| {
            for n in s {
                if n.chars().next().unwrap() == 't' {
                    return true;
                }
            }
            return false;
        })
        .count();
}

fn part2(input: String) -> String {
    let mut nodes: HashMap<String, HashSet<String>> = HashMap::new();
    for line in input.lines() {
        let (l, r) = line.split_once('-').unwrap();
        let (l, r) = (l.to_string(), r.to_string());
        nodes.entry(l.clone()).or_default().insert(r.clone());
        nodes.entry(r).or_default().insert(l);
    }
    let mut biggest_len = 1;
    let mut biggest_set = Vec::new();
    for (node, conns) in &nodes {
        for size in (biggest_len + 1..conns.len() + 1).rev() {
            'outer: for mut cmbn in conns.iter().combinations(size) {
                for (n1, n2) in cmbn.iter().tuple_combinations() {
                    if !nodes[*n1].contains(*n2) {
                        continue 'outer;
                    }
                }
                cmbn.push(&node);
                biggest_len = cmbn.len();
                biggest_set = cmbn;
            }
        }
    }
    biggest_set.sort();
    let mut res = String::new();
    for s in biggest_set {
        res += s;
        res += ",";
    }
    res = res[0..res.len() - 1].to_string();
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
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"
            .to_string();
        assert_eq!(7, part1(input));
    }

    #[test]
    fn p2_1() {
        let input = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"
            .to_string();
        assert_eq!("co,de,ka,ta", part2(input));
    }
}
