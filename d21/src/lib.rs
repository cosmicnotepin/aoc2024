use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;
//use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::time::Instant;

//     v<<AA>A^>AAvA^<A>AAvA^Av<A^>A<A>Av<A^>A<A>Av<A<A>^>AAvA^<A>A
// <v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A
fn to_map(map_s: &str) -> Vec<Vec<char>> {
    return map_s
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
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

fn get_sequence2(seq: &Vec<char>, map: &Vec<Vec<char>>, permutate: bool) -> Vec<Vec<char>> {
    let mut pos = (0, 0);
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == 'A' {
                pos = (r, c);
            }
        }
    }
    let mut subseqs = Vec::new();
    let mut cache: HashMap<((usize, usize), char), ((usize, usize), Vec<Vec<char>>)> =
        HashMap::new();
    for it in seq {
        let (posse, subseq) = get_paths(&pos, &it, map, &mut cache);
        pos = posse;
        subseqs.push(subseq);
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
    let num_keypad = to_map(num_keypad_s);
    let dir_keypad = to_map(dir_keypad_s);
    let mut res = 0;
    for seq in input.lines() {
        let mut permuts = vec![seq.chars().collect::<Vec<_>>()];

        let mut next_permuts = Vec::new();
        for permut in permuts {
            let seq_perms = get_sequence2(&permut, &num_keypad, true);
            next_permuts.extend(seq_perms);
        }
        permuts = next_permuts;
        println!("permuts : {:?}", permuts[0].iter().collect::<String>());

        let mut next_permuts = Vec::new();
        for permut in permuts {
            let seq_perms = get_sequence2(&permut, &dir_keypad, true);
            next_permuts.extend(seq_perms);
        }
        permuts = next_permuts;
        println!("permuts : {:?}", permuts[0].iter().collect::<String>());

        let mut next_permuts = Vec::new();
        for permut in permuts {
            let seq_perms = get_sequence2(&permut, &dir_keypad, false);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_1() {
        let input = "\
456A"
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
    fn p2_1() {
        let input = "".to_string();
        assert_eq!(0, part2(input));
    }
}

//fn get_sequence(seq: &Vec<char>, map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
//    let mut pos = (0, 0);
//    for r in 0..map.len() {
//        for c in 0..map[0].len() {
//            if map[r][c] == 'A' {
//                pos = (r, c);
//            }
//        }
//    }
//    let mut res = Vec::new();
//    let dir2but = HashMap::from([
//        ((0, 0), '_'),
//        ((-1, 0), '^'),
//        ((0, 1), '>'),
//        ((1, 0), 'v'),
//        ((0, -1), '<'),
//        ((-1, -1), 'A'),
//    ]);
//    for it in seq {
//        //println!("it : {:?}", it);
//        let mut todo = VecDeque::new();
//        todo.push_back((pos, vec![]));
//        let mut visited = HashSet::new();
//        'outer: loop {
//            let ((r, c), path) = todo.pop_front().unwrap();
//            let dirs = vec![(0, 0), (-1, 0), (0, 1), (1, 0), (0, -1)];
//            for (rd, cd) in dirs {
//                let (rn, cn) = (((r as isize) + rd) as usize, ((c as isize) + cd) as usize);
//                let atn = &map[rn][cn];
//                if *atn == '#' || !visited.insert((rn, cn)) {
//                    continue;
//                }
//                let mut path = path.clone();
//                if (rd, cd) != (0, 0) {
//                    path.push(dir2but[&(rd, cd)]);
//                }
//                if atn == it {
//                    res.push(path);
//                    pos = (rn, cn);
//                    break 'outer;
//                } else {
//                    todo.push_back(((rn, cn), path));
//                }
//            }
//        }
//    }
//    return res;
//}
//
//fn get_permutations(one_permut: Vec<Vec<char>>) -> Vec<Vec<char>> {
//    let sub_permuts = one_permut
//        .iter()
//        .map(|cs| {
//            cs.iter()
//                .cloned()
//                .permutations(cs.len())
//                .collect::<HashSet<Vec<char>>>()
//        })
//        .collect::<Vec<_>>();
//    let permuts = sub_permuts
//        .iter()
//        .multi_cartesian_product()
//        .collect::<Vec<_>>();
//    let mut res = Vec::new();
//    for permut in permuts {
//        let mut rp = Vec::new();
//        for sp in permut {
//            rp.extend(sp);
//            rp.push('A');
//        }
//        res.push(rp);
//    }
//    return res;
//}
