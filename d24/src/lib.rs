use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fs;
use std::time::Instant;

fn part1(input: String) -> usize {
    let (wires_s, gates_s) = input.split_once("\n\n").unwrap();
    let mut wires = HashMap::new();
    for wire_s in wires_s.lines() {
        let (id_s, val_s) = wire_s.split_once(": ").unwrap();
        wires.insert(id_s, val_s.parse::<usize>().unwrap());
    }
    let mut todo = VecDeque::new();
    for gate_s in gates_s.lines() {
        let v: Vec<&str> = gate_s.split(' ').collect();
        todo.push_back(v);
    }
    while todo.len() > 0 {
        let cur = todo.pop_front().unwrap();
        if let Some(in1) = wires.get(cur[0]) {
            if let Some(in2) = wires.get(cur[2]) {
                let res = match cur[1] {
                    "AND" => in1 & in2,
                    "OR" => in1 | in2,
                    "XOR" => in1 ^ in2,
                    _default => panic!(),
                };
                wires.insert(cur[4], res);
                continue;
            }
        }
        todo.push_back(cur);
    }
    println!("wires: {:?}", wires);
    let wires: Vec<(usize, usize)> = wires
        .iter()
        .filter(|(k, _v)| k.chars().next().unwrap() == 'z')
        .map(|(k, v)| (k[1..].parse::<usize>().unwrap(), *v))
        .collect();
    println!("wires: {:?}", wires);
    let mut res = 0;
    for (shift, val) in wires {
        res += val << shift;
    }
    return res;
}

fn part2(input: String) -> i32 {
    let (wires_s, gates_s) = input.split_once("\n\n").unwrap();
    let mut wires = HashMap::new();
    for wire_s in wires_s.lines() {
        let (id_s, val_s) = wire_s.split_once(": ").unwrap();
        wires.insert(id_s, val_s.parse::<usize>().unwrap());
    }
    let mut todo = Vec::new();
    let mut graph_s = String::from("digraph {\n");
    let mut edges_s = String::from("");
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    let mut zs = Vec::new();
    let mut ands = Vec::new();
    let mut ors = Vec::new();
    let mut xors = Vec::new();
    let swaps = HashMap::from([
        ("nnf", "z09"),
        ("z09", "nnf"),
        ("nhs", "z20"),
        ("z20", "nhs"),
        ("kqh", "ddn"),
        ("ddn", "kqh"),
        ("z34", "wrc"),
        ("wrc", "z34"),
    ]);
    //ddn,kqh,nhs,nnf,wrc,z09,z20,z34
    //let swaps: HashMap<&str, &str> = HashMap::new();
    for gate_s in gates_s.lines() {
        let mut v: Vec<&str> = gate_s.split(' ').collect();
        if swaps.contains_key(v[4]) {
            v[4] = swaps[v[4]];
        }
        todo.push(v);
    }
    todo.sort_by_key(|el| el[4]);
    todo.reverse();
    for v in todo {
        for n in [v[0], v[2], v[4]] {
            match n.chars().next().unwrap() {
                'z' => zs.push(n),
                'x' => {
                    if !xs.contains(&n) {
                        xs.push(n);
                    }
                }
                'y' => {
                    if !ys.contains(&n) {
                        ys.push(n);
                    }
                }
                _other => (),
            }
        }
        for n in [v[4]] {
            match n.chars().next().unwrap() {
                'z' => (),
                _other => match v[1] {
                    "AND" => ands.push(n),
                    "OR" => ors.push(n),
                    "XOR" => xors.push(n),
                    _other => panic!(),
                },
            }
        }
        edges_s += &format!("{} -> {}\n", v[0], v[4]);
        edges_s += &format!("{} -> {}\n", v[2], v[4]);
    }
    xs.sort();
    ys.sort();
    zs.sort();

    //for sgn in [&xs, &ys, &zs] {
    //    for n in sgn {
    //        graph_s += &format!("{}\n", n);
    //    }
    //}
    graph_s += &edges_s;
    for sgn in [ors, ands, xors] {
        graph_s += "subgraph {\n  rank = same;";
        for n in sgn {
            graph_s += &format!(" {};", n);
        }
        graph_s += "\n}\n";
    }
    graph_s += "subgraph {\n  rank = same;\n";
    for (x, y) in xs.iter().zip(ys) {
        graph_s += &format!("{}->{}->", x, y);
    }
    graph_s.pop();
    graph_s.pop();
    graph_s += ";\n";
    graph_s += "rankdir=LR;\n";
    graph_s += "}\n";

    for sgn in [zs] {
        graph_s += "subgraph {\n  rank = same;\n";
        for n in sgn {
            graph_s += &format!("{}->", n);
        }
        graph_s.pop();
        graph_s.pop();
        graph_s += ";\n";
        graph_s += "rankdir=LR;\n";
        graph_s += "}\n";
    }
    graph_s += "}";
    let _ = fs::write("graph", graph_s);
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
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"
            .to_string();
        assert_eq!(4, part1(input));
    }

    #[test]
    fn p1_2() {
        let input = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"
            .to_string();
        assert_eq!(2024, part1(input));
    }

    #[test]
    fn p2_2() {
        let input = "\
x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00"
            .to_string();
        assert_eq!(0, part2(input));
    }
}
//fn part2(input: String) -> i32 {
//    let (wires_s, gates_s) = input.split_once("\n\n").unwrap();
//    let mut wires = HashMap::new();
//    for wire_s in wires_s.lines() {
//        let (id_s, val_s) = wire_s.split_once(": ").unwrap();
//        wires.insert(id_s, val_s.parse::<usize>().unwrap());
//    }
//    let mut todo = VecDeque::new();
//    let mut outputs = Vec::new();
//    for gate_s in gates_s.lines() {
//        let v: Vec<&str> = gate_s.split(' ').collect();
//        outputs.push(v[v.len() - 1]);
//        todo.push_back(v);
//    }
//    println!("outputs: {:?}", outputs);
//    let blah = outputs
//        .iter()
//        .tuple_combinations()
//        .collect::<Vec<(_, _, _, _, _, _, _, _)>>();
//    //let blah = blah
//    //    .iter().map(|cb| cb.combinations(2)
//    //    .tuple_combinations()
//    //    //.filter(|((a, b), (c, d), (e, f), (g, h))| **h == "z00")
//    //    .collect::<Vec<(_, _, _, _)>>();
//    //println!("blah : {:?}", blah);
//    println!("blah.len() : {:?}", blah.len());
//    while todo.len() > 0 {
//        let cur = todo.pop_front().unwrap();
//        if let Some(in1) = wires.get(cur[0]) {
//            if let Some(in2) = wires.get(cur[2]) {
//                let res = match cur[1] {
//                    "AND" => in1 & in2,
//                    "OR" => in1 | in2,
//                    "XOR" => in1 ^ in2,
//                    _default => panic!(),
//                };
//                wires.insert(cur[4], res);
//                continue;
//            }
//        }
//        todo.push_back(cur);
//    }
//    return 0;
//}
//
