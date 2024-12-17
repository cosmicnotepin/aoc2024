use std::collections::BinaryHeap;
use std::error::Error;
use std::fs;
use std::time::Instant;

#[allow(dead_code)]
fn print_map(
    map: &Vec<Vec<(char, [isize; 4])>>,
    todo: &BinaryHeap<(isize, (usize, usize), usize)>,
) {
    println!();
    for (row_i, row) in map.iter().enumerate() {
        for (col_i, col) in row.iter().enumerate() {
            if let Some(todo_rank) =
                todo.iter()
                    .position(|(_score, (row_td_i, col_td_i), _dir_td)| {
                        row_i == *row_td_i && col_i == *col_td_i
                    })
            {
                print!("{}", todo_rank);
            } else {
                print!("{}", col.0);
            }
        }
        println!();
    }
}

fn part1(input: String) -> isize {
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut pos = (0, 0);
    let mut map: Vec<Vec<(char, [isize; 4])>> = Vec::new();
    let mut todo: BinaryHeap<(isize, (usize, usize), usize)> = BinaryHeap::new();
    for (row_i, row) in input.lines().enumerate() {
        let mut new_row: Vec<(char, [isize; 4])> = Vec::new();
        for (col_i, col) in row.chars().enumerate() {
            if col == 'S' {
                todo.push((0, (row_i, col_i), 1));
                pos.0 = row_i;
                pos.1 = col_i;
                new_row.push(('.', [1000, 0, 1000, 2000]));
            } else {
                new_row.push((col, [isize::MAX, isize::MAX, isize::MAX, isize::MAX]));
            }
        }
        map.push(new_row);
    }

    while todo.len() > 0 {
        print_map(&map, &todo);
        let (score, (row_p, col_p), dir_i) = todo.pop().unwrap();
        let score = -score;
        let mut neighbs = Vec::new();
        neighbs.push((score + 1, dir_i));
        neighbs.push((score + 1001, (dir_i + 1) % 4));
        neighbs.push((score + 1001, (dir_i + 3) % 4));
        for (score, dir_n_i) in neighbs {
            let (row_d, col_d) = dirs[dir_n_i];
            let row_n = ((row_p as isize) + row_d) as usize;
            let col_n = ((col_p as isize) + col_d) as usize;
            let (at_n, mut scores) = &map[row_n][col_n];
            if *at_n == 'E' {
                return score;
            }
            if *at_n == '#' {
                continue;
            }
            let mut doit = false;
            if scores[dir_n_i] > score {
                scores[dir_n_i] = score;
                doit = true;
            }
            if scores[(dir_n_i + 1) % 4] > score + 1000 {
                scores[(dir_n_i + 1) % 4] = score + 1000;
                doit = true;
            }
            if scores[(dir_n_i + 3) % 4] > score + 1000 {
                scores[(dir_n_i + 3) % 4] = score + 1000;
                doit = true;
            }
            if scores[(dir_n_i + 2) % 4] > score + 2000 {
                scores[(dir_n_i + 2) % 4] = score + 2000;
                doit = true;
            }
            if doit {
                todo.push((-score, (row_n, col_n), dir_n_i));
            }
        }
    }
    return 0;
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
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
            .to_string();
        assert_eq!(7036, part1(input));
    }

    #[test]
    fn p1_2() {
        let input = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
            .to_string();
        assert_eq!(11048, part1(input));
    }
}
