use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::time::Instant;

fn part1(input: &String, blink_count: usize) -> usize {
    let mut stones: Vec<usize> = input
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    for _ in 0..blink_count {
        let mut new_stones = Vec::new();
        for stone in &stones {
            if *stone == 0 {
                new_stones.push(1);
            } else if ((stone.ilog10() + 1) % 2) == 0 {
                let digits = (stone.ilog10() + 1) as usize;
                let new_stone = stone / 10usize.pow((digits / 2) as u32);
                new_stones.push(new_stone);
                new_stones.push(stone - (new_stone * 10usize.pow((digits / 2) as u32)));
            } else {
                new_stones.push(stone * 2024);
            }
        }
        stones = new_stones;
    }
    return stones.len();
}

fn blink(blink_count: usize, stone: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(count) = cache.get(&(blink_count, stone)) {
        return *count;
    }
    if blink_count == 0 {
        return 1;
    }

    let mut res = 0;
    if stone == 0 {
        res = blink(blink_count - 1, 1, cache);
    } else if ((stone.ilog10() + 1) % 2) == 0 {
        let digits = (stone.ilog10() + 1) as usize;
        let new_stone = stone / 10usize.pow((digits / 2) as u32);
        res += blink(blink_count - 1, new_stone, cache);
        res += blink(
            blink_count - 1,
            stone - (new_stone * 10usize.pow((digits / 2) as u32)),
            cache,
        );
    } else {
        res = blink(blink_count - 1, stone * 2024, cache);
    }
    cache.insert((blink_count, stone), res);
    return res;
}

fn part2(input: &String, blink_count: usize) -> usize {
    let stones: VecDeque<(usize, usize)> = input
        .trim()
        .split(' ')
        .map(|s| (blink_count, s.parse().unwrap()))
        .collect();
    let mut res = 0;
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    for (blink_count, stone) in stones {
        res += blink(blink_count, stone, &mut cache);
    }
    return res;
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let before1 = Instant::now();
    let input1 = fs::read_to_string("input1")?;
    let p1 = part1(&input1, 25);
    println!("part 1: {} in {:.2?}", p1, before1.elapsed());
    let before2 = Instant::now();
    let input2 = fs::read_to_string("input1")?;
    let p2 = part2(&input2, 75);
    println!("part 2: {} in {:.2?}", p2, before2.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_1() {
        let input = "125 17".to_string();
        assert_eq!(22, part1(&input, 6));
        assert_eq!(55312, part1(&input, 25));
    }

    #[test]
    fn p2_1() {
        let input = "125 17".to_string();
        assert_eq!(22, part2(&input, 6));
        assert_eq!(55312, part2(&input, 25));
    }
}
