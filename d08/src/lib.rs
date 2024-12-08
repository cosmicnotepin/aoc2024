extern crate nalgebra as na;
use itertools::Itertools;
use na::Point2;
use std::cmp::max;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::time::Instant;

fn part2(input: String, harmonics: bool) -> i32 {
    let row_count = input.lines().count() as isize;
    let col_count = input.lines().next().unwrap().chars().count() as isize;
    let mut antenna_types: HashMap<char, Vec<Point2<isize>>> = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            antenna_types
                .entry(c)
                .or_default()
                .push(Point2::new(row as isize, col as isize));
        }
    }
    let mut antinodes = HashSet::new();
    let harms;
    if harmonics {
        harms = 0..max(row_count, col_count);
    } else {
        harms = 1..2;
    }
    for antennas in antenna_types.values() {
        for (a1, a2) in antennas.iter().tuple_combinations() {
            let diff = a2 - a1;
            for m in harms.clone() {
                for c in [a2 + m * diff, a1 - m * diff] {
                    if (0..row_count).contains(&c.x) && (0..col_count).contains(&c.y) {
                        antinodes.insert(c);
                    }
                }
            }
        }
    }
    return antinodes.len() as i32;
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let before1 = Instant::now();
    let input1 = fs::read_to_string("input1")?;
    let p1 = part2(input1, false);
    println!("part 1: {} in {:.2?}", p1, before1.elapsed());
    let before2 = Instant::now();
    let input2 = fs::read_to_string("input1")?;
    let p2 = part2(input2, true);
    println!("part 2: {} in {:.2?}", p2, before2.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_0() {
        let input = "\
..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
.........."
            .to_string();
        assert_eq!(2, part2(input, false));
    }

    #[test]
    fn p1_1() {
        let input = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
            .to_string();
        assert_eq!(14, part2(input, false));
    }

    #[test]
    fn p2_1() {
        let input = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
            .to_string();
        assert_eq!(34, part2(input, true));
    }
}
