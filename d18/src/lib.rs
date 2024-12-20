use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::time::Instant;

fn part1(input: String, byte_count: isize, mem_size: isize) -> isize {
    let bytes: HashSet<(isize, isize)> = input
        .lines()
        .take(byte_count as usize)
        .map(|l| {
            let (col, row) = l.split_once(',').unwrap();
            (row.parse().unwrap(), col.parse().unwrap())
        })
        .collect();
    let mut todo: VecDeque<(isize, (isize, isize))> = VecDeque::new();
    let mut visited = HashSet::new();
    todo.push_front((0, (0, 0)));
    while todo.len() > 0 {
        let (len, (row, col)) = todo.pop_front().unwrap();
        for (row_n, col_n) in [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .iter()
            .map(|(r, c)| (row + r, col + c))
            .filter(|(r, c)| {
                *r >= 0 && *r < mem_size && *c >= 0 && *c < mem_size && !bytes.contains(&(*r, *c))
            })
        {
            if row_n == mem_size - 1 && col_n == mem_size - 1 {
                return len + 1;
            }
            if visited.insert((row_n, col_n)) {
                todo.push_back((len + 1, (row_n, col_n)));
            }
        }
    }
    return input.len().try_into().unwrap();
}

fn part2(input: String, mem_size: isize) -> (isize, isize) {
    let bytes_all: Vec<(isize, isize)> = input
        .lines()
        .map(|l| {
            let (col, row) = l.split_once(',').unwrap();
            (row.parse().unwrap(), col.parse().unwrap())
        })
        .collect();
    'outer: for i in 1..bytes_all.len() {
        let mut todo: VecDeque<(isize, (isize, isize))> = VecDeque::new();
        let bytes: HashSet<(isize, isize)> = HashSet::from_iter(bytes_all.iter().take(i).cloned());
        let mut visited = HashSet::new();
        todo.push_front((0, (0, 0)));
        while todo.len() > 0 {
            let (len, (row, col)) = todo.pop_front().unwrap();
            for (row_n, col_n) in [(-1, 0), (0, 1), (1, 0), (0, -1)]
                .iter()
                .map(|(r, c)| (row + r, col + c))
                .filter(|(r, c)| {
                    *r >= 0
                        && *r < mem_size
                        && *c >= 0
                        && *c < mem_size
                        && !bytes.contains(&(*r, *c))
                })
            {
                if row_n == mem_size - 1 && col_n == mem_size - 1 {
                    continue 'outer;
                }
                if visited.insert((row_n, col_n)) {
                    todo.push_back((len + 1, (row_n, col_n)));
                }
            }
        }
        let (row, col) = bytes_all[i - 1];
        return (col, row);
    }
    return (-1, -1);
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let before1 = Instant::now();
    let input1 = fs::read_to_string("input1")?;
    let p1 = part1(input1, 1024, 71);
    println!("part 1: {} in {:.2?}", p1, before1.elapsed());
    let before2 = Instant::now();
    let input2 = fs::read_to_string("input1")?;
    let p2 = part2(input2, 71);
    println!("part 2: {:?} in {:.2?}", p2, before2.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_1() {
        let input = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"
        .to_string();
        assert_eq!(22, part1(input, 12, 7));
    }

    #[test]
    fn p2_1() {
        let input = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"
        .to_string();
        assert_eq!((6, 1), part2(input, 7));
    }
}
