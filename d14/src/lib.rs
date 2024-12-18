use itertools::Itertools;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::time::Instant;

fn part1(input: String, row_count: isize, col_count: isize) -> usize {
    //let re = Regex::new(r"p=(?<col>\d+),(?<row>\d+) v=(?<col_v>-?\d+),(?<row_v>-?\d)").unwrap();
    let re = Regex::new(r"-?\d+").unwrap();
    let mut quadrants = [0, 0, 0, 0];
    for line in input.lines() {
        let Some((mut col, mut row, col_v, row_v)) = re
            .find_iter(line)
            .map(|n| n.as_str().parse::<isize>().unwrap())
            .collect_tuple()
        else {
            panic!();
        };

        row = (row + 100 * (row_v + row_count)) % row_count;
        col = (col + 100 * (col_v + col_count)) % col_count;
        if col == col_count / 2 || row == row_count / 2 {
            continue;
        }
        quadrants[(row > row_count / 2) as usize + 2 * ((col > col_count / 2) as usize)] += 1;
    }
    return quadrants.iter().product();
}
fn print_map(map: &Vec<Vec<char>>, seconds: isize) {
    println!();
    for row in map {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
    println!();
    println!("{}", seconds);
}
fn part2(input: String, row_count: isize, col_count: isize) -> isize {
    let re = Regex::new(r"-?\d+").unwrap();
    let mut bots: Vec<(isize, isize, isize, isize)> = input
        .lines()
        .map(|l| {
            re.find_iter(l)
                .map(|n| n.as_str().parse::<isize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let mut best_dist = isize::MAX;
    for i in 0..1000000000000000 {
        for (ref mut col, ref mut row, col_v, row_v) in bots.iter_mut() {
            *row = (*row + (*row_v + row_count)) % row_count;
            *col = (*col + (*col_v + col_count)) % col_count;
        }
        let dist: isize = bots
            .iter()
            .tuple_combinations()
            .map(|((r1, c1, _, _), (r2, c2, _, _))| (r1 - r2).abs() + (c1 - c2).abs())
            .sum();
        if dist <= best_dist {
            best_dist = dist;
            let mut map: Vec<Vec<char>> = vec![vec!['.'; col_count as usize]; row_count as usize];
            for (col, row, _col_v, _row_v) in &bots {
                map[*row as usize][*col as usize] = '#';
            }
            print_map(&map, i + 1);
        }
    }

    return 0;
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let before1 = Instant::now();
    let input1 = fs::read_to_string("input1")?;
    let p1 = part1(input1, 103, 101);
    println!("part 1: {} in {:.2?}", p1, before1.elapsed());
    let before2 = Instant::now();
    let input2 = fs::read_to_string("input1")?;
    let p2 = part2(input2, 103, 101);
    println!("part 2: {} in {:.2?}", p2, before2.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_1() {
        let input = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"
            .to_string();
        assert_eq!(12, part1(input, 7, 11));
    }

    #[test]
    fn p2_1() {
        let input = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"
            .to_string();
        assert_eq!(12, part2(input, 7, 11));
    }
}
