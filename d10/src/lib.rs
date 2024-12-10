use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::time::Instant;

fn part1(input: String, part1: bool) -> i32 {
    let mut map: Vec<Vec<u32>> = Vec::new();
    let mut trail_heads: Vec<(isize, isize)> = Vec::new();
    for (row_i, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (col_i, c) in line.chars().enumerate() {
            row.push(c.to_digit(10).unwrap());
            if c == '0' {
                trail_heads.push((row_i as isize, col_i as isize));
            }
        }
        map.push(row);
    }

    let mut res = 0;
    for th in trail_heads {
        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        let mut todo: VecDeque<(isize, isize)> = VecDeque::new();
        todo.push_back(th);
        while todo.len() > 0 {
            let (row, col) = todo.pop_front().unwrap();
            if visited.contains(&(row, col)) {
                continue;
            }
            if part1 {
                visited.insert((row, col));
            }
            if map[row as usize][col as usize] == 9 {
                res += 1;
                continue;
            }
            for neighbour in [(-1, 0), (0, 1), (1, 0), (0, -1)]
                .iter()
                .map(|(r, c)| (r + row, c + col))
                .filter(|&(r, c)| {
                    (0..map.len()).contains(&(r as usize))
                        && (0..map[0].len()).contains(&(c as usize))
                        && !visited.contains(&(r, c))
                        && map[r as usize][c as usize] == map[row as usize][col as usize] + 1
                })
            {
                todo.push_back(neighbour);
            }
        }
    }

    return res;
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let before1 = Instant::now();
    let input1 = fs::read_to_string("input1")?;
    let p1 = part1(input1, true);
    println!("part 1: {} in {:.2?}", p1, before1.elapsed());
    let before2 = Instant::now();
    let input2 = fs::read_to_string("input1")?;
    let p2 = part1(input2, false);
    println!("part 2: {} in {:.2?}", p2, before2.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_1() {
        let input = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            .to_string();
        assert_eq!(36, part1(input, true));
    }

    #[test]
    fn p2_1() {
        let input = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            .to_string();
        assert_eq!(81, part1(input, false));
    }
}
