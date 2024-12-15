use num::integer::gcd;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::time::Instant;

fn part_omg(input: String, part2: bool) -> usize {
    let re = Regex::new(r"\d+").unwrap();
    let mut res = 0;
    for machine_descr in input.split("\n\n") {
        let mut iter = re
            .find_iter(machine_descr)
            .map(|n| n.as_str().parse().unwrap());
        let ax: isize = iter.next().unwrap();
        let ay: isize = iter.next().unwrap();
        let bx: isize = iter.next().unwrap();
        let by: isize = iter.next().unwrap();
        let mut tx: isize = iter.next().unwrap();
        let mut ty: isize = iter.next().unwrap();
        if part2 {
            tx += 10000000000000;
            ty += 10000000000000;
        }
        let a = -ax as f64 / bx as f64;
        let c = tx as f64 / bx as f64;
        let b = -ay as f64 / by as f64;
        let d = ty as f64 / by as f64;
        let a_i = (d - c) / (a - b);
        let b_i = a * a_i + c;
        if (a_i - a_i.round()).abs() < 0.001 && (b_i - b_i.round()).abs() < 0.001 {
            res += 3 * (a_i.round() as usize) + (b_i.round() as usize);
        }
    }
    return res;
}

fn part1(input: String) -> usize {
    let re = Regex::new(r"\d+").unwrap();
    let mut res = 0;
    for machine_descr in input.split("\n\n") {
        let mut iter = re
            .find_iter(machine_descr)
            .map(|n| n.as_str().parse().unwrap());
        let ax: usize = iter.next().unwrap();
        let ay: usize = iter.next().unwrap();
        let bx: usize = iter.next().unwrap();
        let by: usize = iter.next().unwrap();
        let tx: usize = iter.next().unwrap();
        let ty: usize = iter.next().unwrap();
        //this only works because of luck
        for a in 1..101 {
            for b in 1..101 {
                if ax * a + bx * b == tx && ay * a + by * b == ty {
                    res += a * 3 + b;
                }
            }
        }
    }

    return res;
}

fn part2(input: String) -> isize {
    // too high: 191565982542788
    //           111529726074646
    //            83197086729371
    // too low:
    //            37745025344471
    let re = Regex::new(r"\d+").unwrap();
    let mut res = 0;
    for machine_descr in input.split("\n\n") {
        let mut iter = re
            .find_iter(machine_descr)
            .map(|n| n.as_str().parse().unwrap());
        let ax: isize = iter.next().unwrap();
        let ay: isize = iter.next().unwrap();
        let bx: isize = iter.next().unwrap();
        let by: isize = iter.next().unwrap();
        let tx: isize = iter.next().unwrap() + 10000000000000;
        let ty: isize = iter.next().unwrap() + 10000000000000;
        let gdcx = gcd(ax, bx);
        let gdcy = gcd(ay, by);
        if (tx % gdcx) != 0 || (ty % gdcy) != 0 {
            println!("no");
            continue;
        } else {
            println!("yes");
        }
        let mut sol1 = (0, 0);
        let u = ax / gdcx;
        let v = bx / gdcx;
        let dir;
        if (u as f64) / (v as f64) > 3.0 {
            dir = 1;
            'outer: for a in 0..10000000000000 {
                if (tx - a * ax) % bx == 0 {
                    sol1.0 = a;
                    sol1.1 = (tx - a * ax) / bx;
                    println!("sol: {:?}", sol1);
                    break 'outer;
                }
            }
        } else {
            dir = -1;
            'outer: for b in 0..10000000000000 {
                if (tx - b * bx) % ax == 0 {
                    sol1.0 = (tx - b * bx) / ax;
                    sol1.1 = b;
                    println!("sol: {:?}", sol1);
                    break 'outer;
                }
            }
        }

        let mut k = 0;
        while (sol1.1 - k * u) >= 0
            && (sol1.0 + k * v) >= 0
            && ay * (sol1.0 + k * v) + by * (sol1.1 - k * u) != ty
        {
            k += dir;
        }
        res += 3 * (sol1.0 + k * v) + (sol1.1 - k * u);
    }
    return res as isize;
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let before1 = Instant::now();
    let input1 = fs::read_to_string("input1")?;
    let p1 = part_omg(input1, false);
    println!("part 1: {} in {:.2?}", p1, before1.elapsed());
    let before2 = Instant::now();
    let input2 = fs::read_to_string("input1")?;
    let p2 = part_omg(input2, true);
    println!("part 2: {} in {:.2?}", p2, before2.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    //    #[test]
    //    fn p1_1() {
    //        let input = "\
    //Button A: X+94, Y+34
    //Button B: X+22, Y+67
    //Prize: X=8400, Y=5400
    //
    //Button A: X+26, Y+66
    //Button B: X+67, Y+21
    //Prize: X=12748, Y=12176
    //
    //Button A: X+17, Y+86
    //Button B: X+84, Y+37
    //Prize: X=7870, Y=6450
    //
    //Button A: X+69, Y+23
    //Button B: X+27, Y+71
    //Prize: X=18641, Y=10279"
    //            .to_string();
    //        assert_eq!(480, part1(input));
    //    }

    #[test]
    fn pomg_1() {
        let input = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
            .to_string();
        assert_eq!(481, part_omg(input, false));
    }

    #[test]
    fn p2_1() {
        let input = "\
    Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=8400, Y=5400

    Button A: X+26, Y+66
    Button B: X+67, Y+21
    Prize: X=12748, Y=12176

    Button A: X+17, Y+86
    Button B: X+84, Y+37
    Prize: X=7870, Y=6450

    Button A: X+69, Y+23
    Button B: X+27, Y+71
    Prize: X=18641, Y=10279"
            .to_string();
        assert_eq!(480, part_omg(input, true));
    }
}
