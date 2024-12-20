use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::time::Instant;

fn build(design: &str, towels: &Vec<&str>, cache: &mut HashMap<String, usize>) -> usize {
    if let Some(res) = cache.get(design) {
        return *res;
    }
    if design.len() == 0 {
        return 1;
    }
    for towel in towels {
        if towel.len() <= design.len()
            && design[..towel.len()] == **towel
            && build(&design[towel.len()..], towels, cache) == 1
        {
            cache.insert(design.to_string(), 1);
            return 1;
        }
    }
    cache.insert(design.to_string(), 0);
    return 0;
}

fn build2(design: &str, towels: &Vec<&str>, cache: &mut HashMap<String, usize>) -> usize {
    if let Some(res) = cache.get(design) {
        return *res;
    }
    if design.len() == 0 {
        return 1;
    }
    let mut res = 0;
    for towel in towels {
        if towel.len() <= design.len() && design[..towel.len()] == **towel {
            res += build2(&design[towel.len()..], towels, cache);
        }
    }
    cache.insert(design.to_string(), res);
    return res;
}

fn part1(input: String) -> usize {
    let (towels_s, designs_s) = input.split_once("\n\n").unwrap();
    let towels = towels_s.split(", ").collect::<Vec<_>>();
    let designs = designs_s.lines().collect::<Vec<_>>();
    let mut res = 0;
    let mut cache: HashMap<String, usize> = HashMap::new();
    for design in designs {
        res += build(&design, &towels, &mut cache);
    }
    return res;
}

fn part2(input: String) -> usize {
    let (towels_s, designs_s) = input.split_once("\n\n").unwrap();
    let towels = towels_s.split(", ").collect::<Vec<_>>();
    let designs = designs_s.lines().collect::<Vec<_>>();
    let mut res = 0;
    let mut cache: HashMap<String, usize> = HashMap::new();
    for design in designs {
        res += build2(&design, &towels, &mut cache);
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
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
            .to_string();
        assert_eq!(6, part1(input));
    }

    #[test]
    fn p2_1() {
        let input = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
            .to_string();
        assert_eq!(16, part2(input));
    }
}
