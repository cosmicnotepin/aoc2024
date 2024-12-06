extern crate nalgebra as na;
use itertools::Itertools;
use na::{Point2, Vector2};
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::time::Instant;

enum PathType {
    Looping,
    Escape,
}

fn check_path(pos: &Point2<isize>, map: &Vec<Vec<char>>) -> (Vec<(usize, usize)>, PathType) {
    let dirs = [
        Vector2::new(-1, 0),
        Vector2::new(0, 1),
        Vector2::new(1, 0),
        Vector2::new(0, -1),
    ];
    let row_count = map.len();
    let col_count = map[0].len();
    let mut dir_i = 0;
    let mut pos = pos.clone();
    let mut visited = vec![vec![[false; 4]; col_count as usize]; row_count as usize];
    visited[pos.x as usize][pos.y as usize][dir_i] = true;
    //print_situation(&row_count, &col_count, &map, &visited);
    loop {
        let ahead = pos + dirs[dir_i];
        if ahead.x < 0
            || ahead.x as usize > row_count - 1
            || ahead.y < 0
            || ahead.y as usize > col_count - 1
        {
            let path = (0..row_count)
                .cartesian_product(0..col_count)
                .filter(|&(ri, ci)| visited[ri][ci].iter().any(|&a| a == true))
                .collect();
            return (path, PathType::Escape);
        }
        if visited[ahead.x as usize][ahead.y as usize][dir_i] {
            let path = (0..row_count)
                .cartesian_product(0..col_count)
                .filter(|&(ri, ci)| visited[ri][ci].iter().any(|&a| a == true))
                .collect();
            return (path, PathType::Looping);
        }

        let at_ahead = map[ahead.x as usize][ahead.y as usize];
        match at_ahead {
            '.' => {
                pos = ahead;
                visited[pos.x as usize][pos.y as usize][dir_i] = true;
            }
            '#' => dir_i = (dir_i + 1) % 4,
            other => panic!("dafuck: {}", other),
        }
    }
}

#[allow(dead_code)]
fn print_situation(
    row_count: &isize,
    col_count: &isize,
    map: &HashMap<Point2<isize>, char>,
    visited: &HashSet<Point2<isize>>,
) {
    println!();
    println!();
    for row in 0..*row_count {
        for col in 0..*col_count {
            let coords = Point2::new(row, col);
            if visited.contains(&coords) {
                print!("X");
            } else {
                print!("{}", map.get(&coords).unwrap());
            }
        }
        println!();
    }
}

fn parse(input: String) -> (Vec<Vec<char>>, Point2<isize>) {
    let mut pos: Point2<isize> = Point2::new(0, 0);
    let mut map: Vec<Vec<char>> = Vec::new();
    for (ri, row) in input.lines().enumerate() {
        map.push(Vec::new());
        for (ci, col) in row.chars().enumerate() {
            let coords = Point2::new(ri as isize, ci as isize);
            if col == '^' {
                pos = coords.clone();
                map[ri].push('.');
            } else {
                map[ri].push(col);
            }
        }
    }
    (map, pos)
}

fn part1(input: String) -> i32 {
    let (map, pos) = parse(input);
    let (path, _pathtype) = check_path(&pos, &map);
    path.len() as i32
}

fn part2(input: String) -> i32 {
    let (mut map, pos) = parse(input);
    let mut res = 0;
    let (path, _) = check_path(&pos, &map);
    for (ri, ci) in path {
        if map[ri][ci] == '#' || pos.x as usize == ri && pos.y as usize == ci {
            continue;
        }
        map[ri][ci] = '#';
        let (_, pathtype) = check_path(&pos, &map);
        match pathtype {
            PathType::Looping => res += 1,
            PathType::Escape => (),
        }
        map[ri][ci] = '.';
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
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            .to_string();
        assert_eq!(41, part1(input));
    }

    #[test]
    fn p2_1() {
        let input = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            .to_string();
        assert_eq!(6, part2(input));
    }
}
