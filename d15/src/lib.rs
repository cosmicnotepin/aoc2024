use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::time::Instant;

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<char>>) {
    println!();
    for row in map {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

fn push(pos: &(usize, usize), dir: &(isize, isize), map: &mut Vec<Vec<char>>) -> bool {
    let row_n = (pos.0 as isize + dir.0) as usize;
    let col_n = (pos.1 as isize + dir.1) as usize;
    let at_n = map[row_n][col_n];
    if at_n == '.' || (at_n == 'O' && push(&(row_n, col_n), &dir, map)) {
        map[row_n][col_n] = map[pos.0][pos.1];
        map[pos.0][pos.1] = '.';
        return true;
    }
    return false;
}

fn part1(input: String) -> usize {
    let (map_s, moves_s) = input.split_once("\n\n").unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut pos = (0, 0);
    for (row_i, row) in map_s.lines().enumerate() {
        let mut new_row = Vec::new();
        for (col_i, col) in row.chars().enumerate() {
            if col == '@' {
                pos = (row_i, col_i);
                new_row.push('@')
            } else {
                new_row.push(col)
            }
        }
        map.push(new_row);
    }

    let char2dir = HashMap::from([('<', (0, -1)), ('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0))]);
    for mov in moves_s.chars().filter(|&c| c != '\n') {
        let (row_m, col_m) = &char2dir[&mov];
        if push(&pos, &char2dir[&mov], &mut map) {
            pos.0 = ((pos.0 as isize) + row_m) as usize;
            pos.1 = ((pos.1 as isize) + col_m) as usize;
        }
    }

    let mut res = 0;
    for (row_i, row) in map.iter().enumerate() {
        for (col_i, col) in row.iter().enumerate() {
            if *col == 'O' {
                res += row_i * 100 + col_i;
            }
        }
    }

    return res;
}

fn push2able(pos: &(usize, usize), dir: &(isize, isize), map: &mut Vec<Vec<char>>) -> bool {
    let row_n = (pos.0 as isize + dir.0) as usize;
    let col_n = (pos.1 as isize + dir.1) as usize;
    let at_n = map[row_n][col_n];
    if at_n == '.' {
        return true;
    }
    if at_n == '#' {
        return false;
    }
    if dir.0 == 0 {
        return push2able(&(row_n, col_n), dir, map);
    } else {
        let offset;
        if at_n == '[' {
            offset = 1;
        } else {
            offset = -1;
        }
        return push2able(&(row_n, col_n), dir, map)
            && push2able(&(row_n, ((col_n as isize) + offset) as usize), dir, map);
    }
}

fn push2(pos: &(usize, usize), dir: &(isize, isize), map: &mut Vec<Vec<char>>) -> bool {
    let row_n = (pos.0 as isize + dir.0) as usize;
    let col_n = (pos.1 as isize + dir.1) as usize;
    let at_n = map[row_n][col_n];
    let mut res = false;
    if at_n == '.' {
        res = true;
    } else if at_n == '#' {
        res = false;
    } else if dir.0 == 0 {
        if push2able(&(row_n, col_n), dir, map) {
            push2(&(row_n, col_n), dir, map);
            res = true;
        }
    } else {
        let offset;
        if at_n == '[' {
            offset = 1;
        } else {
            offset = -1;
        }
        let col_n_offset = ((col_n as isize) + offset) as usize;

        if push2able(&(row_n, col_n), dir, map) && push2able(&(row_n, col_n_offset), dir, map) {
            push2(&(row_n, col_n), dir, map);
            push2(&(row_n, col_n_offset), dir, map);
            res = true;
        }
    }
    if res == true {
        map[row_n][col_n] = map[pos.0][pos.1];
        map[pos.0][pos.1] = '.';
    }
    return res;
}

fn part2(input: String) -> usize {
    let (map_s, moves_s) = input.split_once("\n\n").unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut pos = (0, 0);
    for (row_i, row) in map_s.lines().enumerate() {
        let mut new_row = Vec::new();
        for (col_i, col) in row.chars().enumerate() {
            if col == '@' {
                pos = (row_i, ((col_i as isize) * 2) as usize);
                new_row.push('@');
                new_row.push('.');
            } else if col == 'O' {
                new_row.push('[');
                new_row.push(']');
            } else {
                new_row.push(col);
                new_row.push(col);
            }
        }
        map.push(new_row);
    }

    let char2dir = HashMap::from([('<', (0, -1)), ('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0))]);
    for mov in moves_s.chars().filter(|&c| c != '\n') {
        let (row_m, col_m) = &char2dir[&mov];
        if push2(&pos, &char2dir[&mov], &mut map) {
            pos.0 = ((pos.0 as isize) + row_m) as usize;
            pos.1 = ((pos.1 as isize) + col_m) as usize;
        }
    }

    let mut res = 0;
    for (row_i, row) in map.iter().enumerate() {
        for (col_i, col) in row.iter().enumerate() {
            if *col == '[' {
                res += row_i * 100 + col_i;
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
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"
            .to_string();
        assert_eq!(2028, part1(input));
    }

    #[test]
    fn p1_2() {
        let input = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            .to_string();
        assert_eq!(10092, part1(input));
    }

    #[test]
    fn p2_1() {
        let input = "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"
            .to_string();
        assert_eq!(618, part2(input));
    }

    #[test]
    fn p2_2() {
        let input = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            .to_string();
        assert_eq!(9021, part2(input));
    }
}
