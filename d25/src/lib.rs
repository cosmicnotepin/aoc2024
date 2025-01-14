use std::error::Error;
use std::fs;
use std::time::Instant;

fn part1(input: String) -> i32 {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for obj_s in input.split("\n\n") {
        let mut lock = false;
        let mut obj = [6; 5];
        for (ri, row) in obj_s.lines().enumerate() {
            let row_v = row.chars().collect::<Vec<_>>();
            if ri == 0 && row_v[0] == '#' {
                lock = true;
            }
            if lock {
                for (ci, col) in row_v.iter().enumerate() {
                    if obj[ci] == 6 && *col == '.' {
                        obj[ci] = ri - 1;
                    }
                }
            } else {
                for (ci, col) in row_v.iter().enumerate() {
                    if obj[ci] == 6 && *col == '#' {
                        obj[ci] = 6 - ri;
                    }
                }
            }
        }
        if lock {
            locks.push(obj);
        } else {
            keys.push(obj);
        }
    }
    let mut res = 0;
    for key in keys {
        'll: for lock in &locks {
            for (k, l) in key.iter().zip(lock.iter()) {
                if k + l > 5 {
                    continue 'll;
                }
            }
            res += 1;
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
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"
            .to_string();
        assert_eq!(3, part1(input));
    }

    #[test]
    fn p2_1() {
        let input = "".to_string();
        assert_eq!(0, part2(input));
    }
}
