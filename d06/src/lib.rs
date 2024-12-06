extern crate nalgebra as na;
use na::{Point2, Vector2};
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

enum PathType {
    Looping,
    Escape,
}

fn check_path(
    pos: &Point2<isize>,
    map: &HashMap<Point2<isize>, char>,
    extra_obstruction: Option<Point2<isize>>,
    row_count: isize,
    col_count: isize,
) -> (usize, PathType) {
    let dirs = [
        Vector2::new(-1, 0),
        Vector2::new(0, 1),
        Vector2::new(1, 0),
        Vector2::new(0, -1),
    ];
    let mut dir_i = 0;
    let mut pos = pos.clone();
    let mut visited: HashSet<Point2<isize>> = HashSet::new();
    let mut visited_dir: HashSet<(Point2<isize>, usize)> = HashSet::new();
    visited.insert(pos.clone());
    //print_situation(&row_count, &col_count, &map, &visited);
    loop {
        let ahead = pos + dirs[dir_i];
        if !map.contains_key(&ahead) {
            return (visited.len(), PathType::Escape);
        }
        if visited_dir.contains(&(ahead, dir_i)) {
            return (visited.len(), PathType::Looping);
        }
        if let Some(extra_obstruction) = extra_obstruction {
            if ahead == extra_obstruction {
                dir_i = (dir_i + 1) % 4;
                continue;
            }
        }

        let at_ahead = map.get(&ahead).unwrap();
        match at_ahead {
            '.' => {
                pos = ahead;
                visited.insert(ahead.clone());
                visited_dir.insert((ahead.clone(), dir_i));
            }
            '#' => dir_i = (dir_i + 1) % 4,
            other => panic!("dafuck: {}", other),
        }
    }
}

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

fn parse(input: String) -> (HashMap<Point2<isize>, char>, Point2<isize>, isize, isize) {
    let mut map: HashMap<Point2<isize>, char> = HashMap::new();
    let mut pos: Point2<isize> = Point2::new(0, 0);
    let row_count = input.lines().count() as isize;
    let col_count = input.lines().next().unwrap().chars().count() as isize;

    for (ri, row) in input.lines().enumerate() {
        for (ci, col) in row.chars().enumerate() {
            let coords = Point2::new(ri as isize, ci as isize);
            if col == '^' {
                pos = coords.clone();
                map.insert(coords, '.');
            } else {
                map.insert(coords, col);
            }
        }
    }
    (map, pos, row_count, col_count)
}

fn part1(input: String) -> i32 {
    let (map, pos, row_count, col_count) = parse(input);
    let (pathlen, _pathtype) = check_path(&pos, &map, None, row_count, col_count);
    pathlen as i32
}

fn part2(input: String) -> i32 {
    let (map, pos, row_count, col_count) = parse(input);
    let mut res = 0;
    for row in 0..row_count {
        for col in 0..col_count {
            let coords = Point2::new(row, col);
            if *(map.get(&coords).unwrap()) == '#' || coords == pos {
                continue;
            }
            let (_pathlen, pathtype) = check_path(&pos, &map, Some(coords), row_count, col_count);
            match pathtype {
                PathType::Looping => res += 1,
                PathType::Escape => (),
            }
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
