use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::time::Instant;

fn ff(
    row: usize,
    col: usize,
    new_id: usize,
    map: &mut Vec<Vec<(char, usize)>>,
    p1: bool,
) -> (usize, usize) {
    let mut fences: HashSet<((isize, isize), (isize, isize))> = HashSet::new();
    let mut todo = VecDeque::new();
    todo.push_front((row as isize, col as isize));
    map[row][col].1 = new_id;
    let (plant, _) = map[row][col];
    let row_count = map.len() as isize;
    let col_count = map[0].len() as isize;
    let mut fields = 1;
    while todo.len() > 0 {
        let (row, col) = todo.pop_front().unwrap();
        for (ro, co) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let nr = row + ro;
            let nc = col + co;
            if !(0..row_count).contains(&nr) || !(0..col_count).contains(&nc) {
                fences.insert(((row, col), (ro, co)));
                continue;
            }
            let (n_plant, ref mut n_id) = map[nr as usize][nc as usize];
            if n_plant != plant {
                fences.insert(((row, col), (ro, co)));
                continue;
            }
            if *n_id != new_id {
                *n_id = new_id;
                fields += 1;
                todo.push_back((nr, nc));
            }
        }
    }
    if p1 {
        return (fields, fences.len());
    }

    let mut side_count = 0;
    while fences.len() > 0 {
        let mut todo: VecDeque<((isize, isize), (isize, isize))> = VecDeque::new();
        let fence = fences.iter().next().unwrap().clone();
        side_count += 1;
        fences.remove(&fence);
        todo.push_front(fence);
        while todo.len() > 0 {
            let ((r, c), (ro, co)) = todo.pop_front().unwrap();
            for n_f in [((r - co, c + ro), (ro, co)), ((r + co, c - ro), (ro, co))] {
                if fences.contains(&n_f) {
                    todo.push_back(n_f);
                    fences.remove(&n_f);
                }
            }
        }
    }
    return (fields, side_count);
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
                let (fields, fences) = ff(r, c, init_id, &mut map, true);
                init_id += 1;
                res += fields * fences;
            }
        }
    }
    return res;
}

fn part2(input: String) -> usize {
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
                let (fields, sides) = ff(r, c, init_id, &mut map, false);
                init_id += 1;
                res += fields * sides;
            }
        }
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
        let input = "\
AAAA
BBCD
BBCC
EEEC"
            .to_string();
        assert_eq!(80, part2(input));
    }

    #[test]
    fn p2_2() {
        let input = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"
            .to_string();
        assert_eq!(436, part2(input));
    }

    #[test]
    fn p2_3() {
        let input = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"
            .to_string();
        assert_eq!(368, part2(input));
    }

    #[test]
    fn p2_4() {
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
        assert_eq!(1206, part2(input));
    }
}
