use std::error::Error;
use std::fs;
use std::time::Instant;

fn part1(a: usize) -> i32 {
    let mut a = a;
    let mut b;
    let mut c;
    let mut out = String::new();
    while a != 0 {
        b = a % 8;
        b ^= 3;
        c = a / (2usize.pow(b as u32));
        a = a / 8;
        b ^= c;
        b ^= 5;
        out.push_str(&(b % 8).to_string());
        out.push(',');
    }
    println!("out: {:?}", out);
    return 0;
}

fn part2(input: String) -> usize {
    //too low: 216216791768685
    let mut a = 0;
    let mut b;
    let mut c;
    let program = "2,4,1,3,7,5,0,3,4,3,1,5,5,5,3,0"
        .split(',')
        .map(|d| d.parse::<usize>().unwrap())
        .rev()
        .collect::<Vec<_>>();
    let mut aati = vec![0; program.len()];
    let mut prog_i = 0;
    println!("program : {:?}", program);
    loop {
        b = a % 8;
        b ^= 3;
        c = a / (2usize.pow(b as u32));
        //a = a / 8;
        b ^= c;
        b ^= 5;
        if (b % 8) != program[prog_i] {
            if aati[prog_i] == 7 {
                while aati[prog_i] == 7 {
                    aati[prog_i] = 0;
                    prog_i -= 1;
                    a /= 8;
                }
            }
            a += 1;
            aati[prog_i] += 1;
            continue;
        }
        if (b % 8) == program[prog_i] {
            println!("a : {:?}", a);
            if prog_i == program.len() - 1 {
                break;
            }
            a *= 8;
            prog_i += 1
        }
    }
    part1(a);
    return a;
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let before1 = Instant::now();
    let input1 = fs::read_to_string("input1")?;
    let p1 = part1(22571680);
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
        let input = "".to_string();
        assert_eq!(0, part1(input));
    }

    #[test]
    fn p2_1() {
        let input = "".to_string();
        assert_eq!(0, part2(input));
    }
}
