use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;
//use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::time::Instant;

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

#[allow(dead_code)]
fn get_paths2(
    pos_start: &(usize, usize),
    target: &char,
    map: &Vec<Vec<char>>,
    cache: &mut HashMap<((usize, usize), char), ((usize, usize), Vec<Vec<char>>)>,
    to_coords: &HashMap<char, (usize, usize)>,
) -> ((usize, usize), Vec<Vec<char>>) {
    if let Some(cached) = cache.get(&(*pos_start, *target)) {
        return cached.clone();
    }
    if map[pos_start.0][pos_start.1] == *target {
        cache.insert(
            (pos_start.clone(), target.clone()),
            (pos_start.clone(), vec![vec![]]),
        );
        return (pos_start.clone(), vec![vec![]]);
    }
    let (rp, cp) = pos_start;
    let (rt, ct) = to_coords[target];
    let mut instrs = Vec::new();
    let rd = rt as isize - *rp as isize;
    let cd = ct as isize - *cp as isize;
    let horiz;
    if cd < 0 {
        horiz = '<';
    } else {
        horiz = '>';
    }
    let vert;
    if rd < 0 {
        vert = '^';
    } else {
        vert = 'v';
    }
    if map.len() == 4 {
        //dir keypad
        if rd < 0 {
            //^ component
            for _ in 0..cd.abs() {
                instrs.push(horiz);
            }
            for _ in 0..rd.abs() {
                instrs.push(vert);
            }
        } else {
            //v or - component
            for _ in 0..rd.abs() {
                instrs.push(vert);
            }
            for _ in 0..cd.abs() {
                instrs.push(horiz);
            }
        }
    } else {
        //num keypad
        if rd < 0 {
            //^ component
            for _ in 0..rd.abs() {
                instrs.push(vert);
            }
            for _ in 0..cd.abs() {
                instrs.push(horiz);
            }
        } else {
            //v or - component
            for _ in 0..cd.abs() {
                instrs.push(horiz);
            }
            for _ in 0..rd.abs() {
                instrs.push(vert);
            }
        }
    }
    return ((rt, ct), vec![instrs]);
}

fn get_paths(
    pos_start: &(usize, usize),
    target: &char,
    map: &Vec<Vec<char>>,
    cache: &mut HashMap<((usize, usize), char), ((usize, usize), Vec<Vec<char>>)>,
) -> ((usize, usize), Vec<Vec<char>>) {
    if let Some(cached) = cache.get(&(*pos_start, *target)) {
        return cached.clone();
    }
    if map[pos_start.0][pos_start.1] == *target {
        cache.insert(
            (pos_start.clone(), target.clone()),
            (pos_start.clone(), vec![vec![]]),
        );
        return (pos_start.clone(), vec![vec![]]);
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
    cache.insert(
        (pos_start.clone(), target.clone()),
        (pos.clone(), subseq.clone()),
    );
    return (pos, subseq);
}

fn get_sequence2(
    seq: &Vec<char>,
    map: &Vec<Vec<char>>,
    permutate: bool,
    to_coords: &HashMap<char, (usize, usize)>,
    new_paths: bool,
) -> Vec<Vec<char>> {
    let mut pos = to_coords[&'A'];
    let mut subseqs = Vec::new();
    let mut cache: HashMap<((usize, usize), char), ((usize, usize), Vec<Vec<char>>)> =
        HashMap::new();
    for it in seq {
        if new_paths {
            let (posse, subseq) = get_paths2(&pos, &it, map, &mut cache, to_coords);
            pos = posse;
            subseqs.push(subseq);
        } else {
            let (posse, subseq) = get_paths(&pos, &it, map, &mut cache);
            pos = posse;
            subseqs.push(subseq);
        }
    }

    if permutate {
        let permuts = subseqs.iter().multi_cartesian_product();
        let mut res = Vec::new();
        for permut in permuts {
            let mut rp = Vec::new();
            for sp in permut {
                rp.extend(sp);
                rp.push('A');
            }
            res.push(rp);
        }
        return res;
    } else {
        let mut res = Vec::new();
        for sp in subseqs {
            res.extend(&sp[0]);
            res.push('A');
        }
        return vec![res];
    }
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
    let new = true;
    for seq in input.lines() {
        let mut permuts = vec![seq.chars().collect::<Vec<_>>()];

        let mut next_permuts = Vec::new();
        for permut in permuts {
            let seq_perms = get_sequence2(&permut, &num_keypad, true, &to_coords_num, new);
            next_permuts.extend(seq_perms);
        }
        permuts = next_permuts;
        println!("permuts : {:?}", permuts[0].iter().collect::<String>());

        let mut next_permuts = Vec::new();
        for permut in permuts {
            let seq_perms = get_sequence2(&permut, &dir_keypad, true, &to_coords_dir, new);
            next_permuts.extend(seq_perms);
        }
        permuts = next_permuts;
        println!("permuts : {:?}", permuts[0].iter().collect::<String>());
        let mut min = usize::MAX;
        let mut min_seq = Vec::new();
        for permut in &permuts {
            if permut.len() < min {
                min = permut.len();
                min_seq = permut.clone();
            }
        }
        println!("min_seq : {:?}", min_seq.iter().collect::<String>());
        println!("min : {:?}", min);

        let mut next_permuts = Vec::new();
        let mut hm: HashMap<Vec<char>, Vec<char>> = HashMap::new();
        for permut in permuts {
            let seq_perms = get_sequence2(&permut, &dir_keypad, false, &to_coords_dir, new);
            for sp in &seq_perms {
                hm.insert(sp.clone(), permut.clone());
            }
            next_permuts.extend(seq_perms);
        }
        permuts = next_permuts;
        println!("permuts : {:?}", permuts[0].iter().collect::<String>());

        let mut min = usize::MAX;
        let mut min_seq = Vec::new();
        for permut in permuts {
            if permut.len() < min {
                min = permut.len();
                min_seq = permut;
            }
        }
        println!("min_seq : {:?}", min_seq.iter().collect::<String>());
        println!("from : {:?}", hm[&min_seq].iter().collect::<String>());
        println!("min : {:?}", min);
        res += min * seq[..seq.len() - 1].parse::<usize>().unwrap();
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
#[allow(dead_code)]
fn ppss(sequences: &Vec<Vec<char>>) {
    for sequence in sequences {
        pps(sequence);
    }
}

#[allow(dead_code)]
fn pps(sequence: &Vec<char>) {
    println!("{}", sequence.iter().collect::<String>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_1() {
        let input = "\
379A"
            .to_string();
        assert_eq!(126384, part1(input));
    }

    #[test]
    fn p1_2() {
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
    fn p1_3() {
        let dir_keypad_s = "\
#####
##^A#
#<v>#
#####";
        let (dir_keypad, to_coords_dir) = to_map(dir_keypad_s);
        let t1 = vec!['<', '<', '^', '^', 'A', '>', '>', 'v', 'v', 'A'];
        let t2 = vec!['^', '^', '<', '<', 'A', 'v', 'v', '>', '>', 'A'];
        //let t1 = vec!['<', '<', '^', '^', 'A', '>', '>', '^', '^', 'A'];
        //let t2 = vec!['^', '^', '<', '<', 'A', 'v', 'v', '>', '>', 'A'];
        let tn_1 = get_sequence2(&t1, &dir_keypad, true, &to_coords_dir, true);
        println!("tn_1");
        ppss(&tn_1);
        let tnn_1 = get_sequence2(&tn_1[0], &dir_keypad, false, &to_coords_dir, true);
        println!("tnn_1");
        ppss(&tnn_1);
        let tn_2 = get_sequence2(&t2, &dir_keypad, true, &to_coords_dir, true);
        println!("tn_2");
        ppss(&tn_2);
        let tnn_2 = get_sequence2(&tn_2[0], &dir_keypad, false, &to_coords_dir, true);
        println!("tnn_2");
        ppss(&tnn_2);

        let to_1 = get_sequence2(&t1, &dir_keypad, true, &to_coords_dir, false);
        println!("to_1");
        ppss(&to_1);
        let mut too1 = Vec::new();
        for t in to_1 {
            let too_1 = get_sequence2(&t, &dir_keypad, false, &to_coords_dir, false);
            too1.extend(too_1);
        }
        println!("too1");
        ppss(&too1);
        let to_2 = get_sequence2(&t2, &dir_keypad, true, &to_coords_dir, false);
        println!("to_2");
        ppss(&to_2);
        let mut too2 = Vec::new();
        for t in to_2 {
            let too_2 = get_sequence2(&t, &dir_keypad, false, &to_coords_dir, false);
            too2.extend(too_2);
        }
        println!("too2");
        ppss(&too2);

        let narf = "<A>A<AAv<AA>>^AvAA^Av<AAA>^A".chars().collect::<Vec<_>>();
        pps(&narf);
        let narf1 = get_sequence2(&narf, &dir_keypad, false, &to_coords_dir, true);
        let narf2 = get_sequence2(&narf, &dir_keypad, false, &to_coords_dir, false);
        ppss(&narf1);
        ppss(&narf2);
        println!("narf2.len(): {:?}", narf2[0].len());
        assert_eq!(0, 1);
    }

    #[test]
    fn p2_1() {
        let input = "".to_string();
        assert_eq!(0, part2(input));
    }
}
