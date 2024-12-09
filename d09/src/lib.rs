use std::error::Error;
use std::fs;
use std::time::Instant;

fn solve(mut files: Vec<(usize, i32)>) -> usize {
    let mut i = files.len() - 1;
    while i > 0 {
        let (size, id) = files[i];
        if id == -1 {
            i -= 1;
            continue;
        }
        if let Some(j) = files[0..i]
            .iter()
            .position(|&(s, id)| id == -1 && size <= s)
        {
            let s = files[j].0;
            files[j] = (size, id);
            files[i] = (size, -1);
            if size < s {
                files.insert(j + 1, (s - size, -1));
            }
        }
        i -= 1;
    }
    files
        .iter()
        .flat_map(|&(s, id)| (0..s).map(move |_| id))
        .enumerate()
        .map(|(i, id)| if id == -1 { 0 } else { i * id as usize })
        .sum()
}

fn part3(input: &str) -> (usize, usize) {
    let mut fs1 = Vec::new();
    let mut fs2 = Vec::new();
    let mut fid = 0;
    for (i, b) in input.bytes().enumerate() {
        let v = if i % 2 == 0 {
            fid += 1;
            fid - 1
        } else {
            -1
        };
        fs1.extend((0..b - b'0').map(|_| (1, v)));
        fs2.push(((b - b'0') as usize, v));
    }
    let p1 = solve(fs1);
    let p2 = solve(fs2);
    (p1, p2)
}

fn part1(input: String) -> isize {
    let map: Vec<(isize, isize)> = input
        .chars()
        .filter(|c| *c != '\n')
        .enumerate()
        .map(|(i, c)| (i as isize, c.to_digit(10).unwrap() as isize))
        .collect();
    let mut front_iter = map.iter();
    let mut back_iter = map.iter().rev();
    let (mut b_i, mut b) = back_iter.next().unwrap();
    let mut res_i: isize = 0;
    let mut res: isize = 0;
    'outer: loop {
        let (f_i, mut f) = front_iter.next().unwrap();
        if b_i == *f_i {
            f = b;
        }
        for _ in 0..f {
            res += res_i * f_i / 2;
            res_i += 1;
            //print!("{:?}", f_i / 2);
        }

        let (_, mut g) = front_iter.next().unwrap();

        while g > 0 {
            if b_i == *f_i {
                break 'outer;
            }
            if b > 0 {
                res += res_i * b_i / 2;
                res_i += 1;
                b -= 1;
                g -= 1;
                //print!("{:?}", b_i / 2);
            } else {
                back_iter.next();
                (b_i, b) = *(back_iter.next().unwrap());
            }
        }
    }
    return res;
}

#[derive(Copy, Clone)]
struct Chunk {
    id: usize,
    size: usize,
    is_file: bool,
    checked: bool,
}

// (file_id|unused_broken_gapid, size, is_file, checked)
fn part2(input: String) -> isize {
    let mut map: Vec<Chunk> = input
        .chars()
        .filter(|c| *c != '\n')
        .enumerate()
        .map(|(i, c)| Chunk {
            id: i / 2,
            size: c.to_digit(10).unwrap() as usize,
            is_file: (i % 2) == 0,
            checked: false,
        })
        .collect();

    let mut i = map.len();
    loop {
        if i == 0 {
            break;
        }
        i -= 1;
        let mut insert_i = map.len();
        let mut move_i = map.len();
        let cb = &(map[i]);
        if !cb.checked && cb.is_file {
            move_i = i;
            for (j, cf) in map.iter().take(i).enumerate() {
                if !cf.is_file && cf.size >= cb.size {
                    insert_i = j;
                    break;
                }
            }
        } else {
            continue;
        }
        if move_i == map.len() {
            break;
        }
        map[move_i].checked = true;
        if insert_i == map.len() {
            continue;
        }
        map[insert_i].id = map[move_i].id;
        let rest = map[insert_i].size - map[move_i].size;
        map[insert_i].size = map[move_i].size;
        map[insert_i].checked = true;
        map[move_i].is_file = false;
        map[insert_i].is_file = true;
        if rest > 0 {
            map.insert(
                insert_i + 1,
                Chunk {
                    id: 0,
                    size: rest,
                    is_file: false,
                    checked: true,
                },
            );
        }
        //for c in map.iter() {
        //    for _ in 0..c.size {
        //        if c.is_file {
        //            print!("{}", c.id);
        //        } else {
        //            print!("{}", ".");
        //        }
        //    }
        //}
        //println!();
    }
    let mut res = 0;
    let mut res_i = 0;
    for c in map {
        for _ in 0..c.size {
            if c.is_file {
                res += res_i * c.id;
                //print!("{}", c.id);
            } else {
                //print!("{}", ".");
            }
            res_i += 1;
        }
    }
    return res as isize;
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

    let before3 = Instant::now();
    let input3 = fs::read_to_string("input1")?;
    let (p3_1, p3_2) = part3(&input3);
    println!("part 3: {},{} in {:.2?}", p3_1, p3_2, before3.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_0() {
        let input = "12345".to_string();
        assert_eq!(60, part1(input));
    }

    #[test]
    fn p1_1() {
        let input = "2333133121414131402".to_string();
        assert_eq!(1928, part1(input));
    }

    #[test]
    fn p2_1() {
        let input = "2333133121414131402".to_string();
        assert_eq!(2858, part2(input));
    }
}
