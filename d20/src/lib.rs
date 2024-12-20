use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::time::Instant;

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<isize>>) {
    for row in map {
        for col in row {
            if *col == -1 {
                print!("{}", "(##)");
            } else {
                print!("({:2})", *col);
            }
        }
        println!();
    }
}
fn part1(input: String) -> i32 {
    let mut map = Vec::new();
    let mut pos = (0, 0);
    let mut goal = (0, 0);
    for (row_i, row_s) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (col_i, col_s) in row_s.chars().enumerate() {
            match col_s {
                '#' => row.push(-1),
                '.' => row.push(-2),
                'S' => {
                    pos = (row_i, col_i);
                    row.push(0)
                }
                'E' => {
                    goal = (row_i, col_i);
                    row.push(-2)
                }
                _ => panic!(),
            }
        }
        map.push(row);
    }
    //print_map(&map);
    let mut path = Vec::new();
    while pos != goal {
        let (row, col) = pos;
        path.push(pos);
        let (row_n, col_n) = [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .iter()
            .map(|(r, c)| (row as isize + r, col as isize + c))
            .filter(|(r, c)| map[*r as usize][*c as usize] == -2)
            .next()
            .unwrap();
        map[row_n as usize][col_n as usize] = map[row][col] + 1;
        pos = (row_n as usize, col_n as usize);
    }
    //print_map(&map);
    //println!("path : {:?}", path);
    let row_count = map.len() as isize;
    let col_count = map[0].len() as isize;
    let mut shortcuts = HashMap::new();

    for (row, col) in path {
        let before = map[row][col];
        for (row_d, col_d) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let (row_w, col_w) = (
                (row as isize + row_d) as usize,
                (col as isize + col_d) as usize,
            );
            if map[row_w][col_w] != -1 {
                continue;
            }
            let (row_j, col_j) = (row as isize + 2 * row_d, col as isize + 2 * col_d);
            if !(0 <= row_j && row_count > row_j && 0 <= col_j && col_count > col_j) {
                continue;
            }
            let after = map[row_j as usize][col_j as usize];
            if after < before {
                continue;
            }
            shortcuts
                .entry((after - before) - 2)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }

    return shortcuts
        .iter()
        .filter(|(saved, _count)| **saved >= 100)
        .map(|(_saved, count)| count)
        .sum();
}

fn part2(input: String) -> i32 {
    return input.len().try_into().unwrap();
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
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"
            .to_string();
        assert_eq!(0, part1(input));
    }

    #[test]
    fn p2_1() {
        let input = "".to_string();
        assert_eq!(0, part2(input));
    }
}
