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
    let bots: Vec<(isize, isize, isize, isize)> = input
        .lines()
        .map(|l| {
            re.find_iter(l)
                .map(|n| n.as_str().parse::<isize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    'outer: for i in 0..1000000000000000 {
        let mut quadrants = [0, 0, 0, 0];
        for (col, row, col_v, row_v) in &bots {
            let row_i = (row + i * (row_v + row_count)) % row_count;
            let col_i = (col + i * (col_v + col_count)) % col_count;
            if col_i != col_count / 2 && row_i != row_count / 2 {
                quadrants
                    [(row_i > row_count / 2) as usize + 2 * ((col_i > col_count / 2) as usize)] +=
                    1;
            }
            //let offset = 25;
            //println!("pos: : {:?}", (row_i, col_i));
            //if col_i > col_count / 2 {
            //    print!("right side: ");
            //    if (row_i - row_count / 2) < (col_i - col_count / 2) - row_count / 2 - offset {
            //        println!("above");
            //        continue 'outer;
            //    }
            //    println!("below");
            //} else {
            //    print!("left side: ");
            //    if (row_i - row_count / 2) < -(col_i - col_count / 2) - row_count / 2 - offset {
            //        println!("above");
            //        continue 'outer;
            //    }
            //    println!("below");
            //}
            //println!("in tree");
        }
        //if !(quadrants[0] == quadrants[2] && quadrants[1] == quadrants[3])
        //    || !(quadrants[0] + 10 < quadrants[1])
        //{
        //    continue 'outer;
        //}
        let mut map: Vec<Vec<char>> = vec![vec!['.'; col_count as usize]; row_count as usize];
        for (col, row, col_v, row_v) in &bots {
            let row_i = ((row + i * (row_v + row_count)) % row_count) as usize;
            let col_i = ((col + i * (col_v + col_count)) % col_count) as usize;
            map[row_i][col_i] = '#';
        }
        print_map(&map, i);
        //return i;
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
