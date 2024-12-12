use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::time::Instant;

fn ff(row: usize, col: usize, new_id: usize, map: &mut Vec<Vec<(char, usize)>>) -> (usize, usize) {
    let fences: HashSet<((usize, usize), (isize, isize))> = HashSet::new();
    let mut todo = VecDeque::new();
    todo.push_front((row, col));
    map[row][col].1 = new_id;
    let (plant, _) = map[row][col];
    let row_count = map.len() as isize;
    let col_count = map[0].len() as isize;
    let mut fence_count = 0;
    let mut fields = 1;
    while todo.len() > 0 {
        let (row, col) = todo.pop_front().unwrap();
        for (nr, nc) in [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .iter()
            .map(|(ro, co)| (row as isize + ro, col as isize + co))
        {
            if !(0..row_count).contains(&nr) || !(0..col_count).contains(&nc) {
                fence_count += 1;
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            let (n_plant, n_id) = map[nr][nc];
            if n_plant != plant {
                fence_count += 1;
                continue;
            }
            if n_id != new_id {
                map[nr][nc].1 = new_id;
                fields += 1;
                todo.push_back((nr, nc));
            }
        }
    }
    return (fields, fence_count);
}

fn part1(input: String) -> usize {
    let mut init_id = 1;
    let mut map: Vec<Vec<(char, usize)>> = input
        .lines()
        .map(|l| l.chars().map(|c| (c, 0)).collect())
        .collect();
    println!("before ff");
    let mut res = 0;
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c].1 == 0 {
                let (fields, fences) = ff(r, c, init_id, &mut map);
                init_id += 1;
                res += fields * fences;
            }
        }
    }
    return res;
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
AAAA
BBCD
BBCC
EEEC"
            .to_string();
        assert_eq!(140, part1(input));
    }

    #[test]
    fn p1_2() {
        let input = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"
            .to_string();
        assert_eq!(772, part1(input));
    }

    #[test]
    fn p1_3() {
        let input = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
            .to_string();
        assert_eq!(1930, part1(input));
    }

    #[test]
    fn p2_1() {
        let input = "".to_string();
        assert_eq!(0, part2(input));
    }
}
