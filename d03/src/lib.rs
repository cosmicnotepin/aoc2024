use regex::Regex;
use std::error::Error;
use std::fs;

fn part1(input: String) -> i32 {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    re.captures_iter(&input)
        .map(|caps| {
            let (_, [n1, n2]) = caps.extract();
            let n1 = n1.parse::<i32>().unwrap();
            let n2 = n2.parse::<i32>().unwrap();
            n1 * n2
        })
        .sum()
}

fn part2(mut input: String) -> i32 {
    let re = Regex::new(r"(?s)don't\(\).*?do\(\)").unwrap();
    while let Some(m) = re.find(&input) {
        input.replace_range(m.range(), "");
    }
    return part1(input);
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let input1 = fs::read_to_string("input1")?;
    println!("part 1: {}", part1(input1));
    let input2 = fs::read_to_string("input1")?;
    println!("part 2: {}", part2(input2));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_1() {
        let input =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))\n".to_string();
        assert_eq!(161, part1(input));
    }

    #[test]
    fn p2_1() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))\n"
            .to_string();
        assert_eq!(48, part2(input));
    }
}
