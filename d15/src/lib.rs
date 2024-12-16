use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::time::Instant;

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<char>>, pos: &(isize, isize)) {
    println!();
    for (row_i, row) in map.iter().enumerate() {
        for (col_i, col) in row.iter().enumerate() {
            if *pos == (row_i as isize, col_i as isize) {
                print!("{}", '@');
            } else {
                print!("{}", col);
            }
        }
        println!();
    }
}

fn push(pos: &(isize, isize), dir: &(isize, isize), map: &mut Vec<Vec<char>>) -> bool {
    let row_n = (pos.0 + dir.0) as usize;
    let col_n = (pos.1 + dir.1) as usize;
    let at_n = map[row_n][col_n];
    if at_n == '.' || (at_n == 'O' && push(&(row_n as isize, col_n as isize), &dir, map)) {
        map[row_n][col_n] = 'O';
        map[pos.0 as usize][pos.1 as usize] = '.';
        return true;
    }
    return false;
}

fn scoot(pos: &mut (isize, isize), dir: &(isize, isize), map: &mut Vec<Vec<char>>) {
    let row_n = (pos.0 + dir.0) as usize;
    let col_n = (pos.1 + dir.1) as usize;
    let at_n = map[row_n][col_n];
    if at_n == '.' || (at_n == 'O' && push(&(row_n as isize, col_n as isize), &dir, map)) {
        pos.0 = row_n as isize;
        pos.1 = col_n as isize;
    }
}

fn part1(input: String) -> usize {
    let (map_s, moves_s) = input.split_once("\n\n").unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut pos = (0, 0);
    for (row_i, row) in map_s.lines().enumerate() {
        let mut new_row = Vec::new();
        for (col_i, col) in row.chars().enumerate() {
            if col == '@' {
                pos = (row_i as isize, col_i as isize);
                new_row.push('.')
            } else {
                new_row.push(col)
            }
        }
        map.push(new_row);
    }
    //print_map(&map, &pos);

    let char2dir = HashMap::from([('<', (0, -1)), ('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0))]);
    for mov in moves_s.chars().filter(|&c| c != '\n') {
        scoot(&mut pos, &char2dir[&mov], &mut map);
        //print_map(&map, &pos);
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
        let input = "".to_string();
        assert_eq!(0, part2(input));
    }
}
