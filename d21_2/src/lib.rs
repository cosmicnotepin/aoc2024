use std::cmp;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fs;
use std::time::Instant;

#[allow(dead_code)]
fn pps(sequence: &Vec<char>) {
    println!(
        "{} : {}",
        sequence.iter().collect::<String>(),
        sequence.len()
    );
}

fn to_map(map_s: &str) -> (Vec<Vec<char>>, HashMap<char, (usize, usize)>) {
    let map = map_s
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut to_coords = HashMap::new();
    for (row_i, row) in map.iter().enumerate() {
        for (col_i, col) in row.iter().enumerate() {
            to_coords.insert(*col, (row_i, col_i));
        }
    }
    return (map, to_coords);
}

fn get_paths(
    pos_start: &(usize, usize),
    target: &char,
    map: &Vec<Vec<char>>,
) -> ((usize, usize), Vec<Vec<char>>) {
    if map[pos_start.0][pos_start.1] == *target {
        return (pos_start.clone(), vec![vec!['A']]);
    }
    let dir2but = HashMap::from([
        ((0, 0), '_'),
        ((-1, 0), '^'),
        ((0, 1), '>'),
        ((1, 0), 'v'),
        ((0, -1), '<'),
        ((-1, -1), 'A'),
    ]);
    let mut pos = *pos_start;
    let mut todo = VecDeque::new();
    todo.push_back((vec![pos], vec![]));
    let mut cur_min = usize::MAX;
    let mut subseq = Vec::new();
    'outer: while todo.len() > 0 {
        let (visited, path) = todo.pop_front().unwrap();
        let (r, c) = visited[visited.len() - 1];
        let dirs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        for (rd, cd) in dirs {
            let (rn, cn) = (((r as isize) + rd) as usize, ((c as isize) + cd) as usize);
            let atn = &map[rn][cn];
            if *atn == '#'
                || (visited.len() > 1 && visited[..visited.len() - 1].contains(&(rn, cn)))
            {
                continue;
            }
            let mut path = path.clone();
            let mut visited = visited.clone();
            path.push(dir2but[&(rd, cd)]);
            visited.push((rn, cn));
            if atn == target {
                cur_min = cmp::min(cur_min, path.len());
                path.push('A');
                subseq.push(path);
                pos = (rn, cn);
                continue 'outer;
            } else {
                if path.len() + 1 < cur_min {
                    todo.push_back((visited, path));
                }
            }
        }
    }
    return (pos, subseq);
}

fn get_sequences(
    seq: &Vec<char>,
    map: &Vec<Vec<char>>,
    to_coords: &HashMap<char, (usize, usize)>,
) -> Vec<Vec<Vec<char>>> {
    let mut pos = to_coords[&'A'];
    let mut subseqs = Vec::new();
    for it in seq {
        let (posse, subseq) = get_paths(&pos, &it, map);
        pos = posse;
        subseqs.push(subseq);
    }
    return subseqs;
}

fn min_exp_size(
    seq: &Vec<char>,
    exps: usize,
    max_exps: usize,
    map_num: &Vec<Vec<char>>,
    to_coords_num: &HashMap<char, (usize, usize)>,
    map_dir: &Vec<Vec<char>>,
    to_coords_dir: &HashMap<char, (usize, usize)>,
) -> usize {
    if exps == max_exps {
        return seq.len();
    }
    let paths: Vec<Vec<Vec<char>>>;
    if exps == 0 {
        paths = get_sequences(seq, map_num, to_coords_num);
    } else {
        paths = get_sequences(seq, map_dir, to_coords_dir);
    }
    let mut res = 0;
    for alts in paths {
        let mut best_alt = usize::MAX;
        for alt in &alts {
            best_alt = cmp::min(
                best_alt,
                min_exp_size(
                    alt,
                    exps + 1,
                    max_exps,
                    map_num,
                    to_coords_num,
                    map_dir,
                    to_coords_dir,
                ),
            );
        }
        res += best_alt;
    }
    return res;
}

fn part1(input: String) -> usize {
    let num_keypad_s = "\
#####
#789#
#456#
#123#
##0A#
#####";
    let dir_keypad_s = "\
#####
##^A#
#<v>#
#####";
    let (num_keypad, to_coords_num) = to_map(num_keypad_s);
    let (dir_keypad, to_coords_dir) = to_map(dir_keypad_s);
    let mut res = 0;
    for code in input.lines() {
        let seqn = code.chars().collect::<Vec<_>>();
        let r1 = min_exp_size(
            &seqn,
            0,
            3,
            &num_keypad,
            &to_coords_num,
            &dir_keypad,
            &to_coords_dir,
        );
        println!("r1 : {:?}", r1);
        res += r1 * code[..code.len() - 1].parse::<usize>().unwrap();
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
029A
980A
179A
456A
379A"
            .to_string();
        assert_eq!(126384, part1(input));
    }

    #[test]
    fn p2_1() {
        let input = "".to_string();
        assert_eq!(0, part2(input));
    }
}
