use std::cell::Cell;
use std::io::Write;

use fnv::FnvHashMap;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum LogicalOp {
    And,
    Or,
    Xor,
}

impl std::fmt::Display for LogicalOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "shape={}",
            match self {
                Self::And => "diamond",
                Self::Or => "box",
                Self::Xor => "hexagon",
            }
        )
    }
}

#[derive(Debug, Clone)]
struct LogicGate<'s> {
    left: &'s str,
    right: &'s str,
    op: LogicalOp,
    generated_output: Cell<Option<bool>>,
}

impl<'s> LogicGate<'s> {
    fn new(left: &'s str, right: &'s str, op: LogicalOp) -> Self {
        Self {
            left,
            right,
            op,
            generated_output: Cell::new(None),
        }
    }

    fn parse_logic_gate(s: &'s str) -> (&'s str, Self) {
        let (operation, target) = s.split_once(" -> ").expect("Could not split on \" -> \"");
        if let Some((left, right)) = operation.split_once(" AND ") {
            return (
                target.trim(),
                Self::new(left.trim(), right.trim(), LogicalOp::And),
            );
        }

        if let Some((left, right)) = operation.split_once(" OR ") {
            return (
                target.trim(),
                Self::new(left.trim(), right.trim(), LogicalOp::Or),
            );
        }

        if let Some((left, right)) = operation.split_once(" XOR ") {
            return (
                target.trim(),
                Self::new(left.trim(), right.trim(), LogicalOp::Xor),
            );
        }

        panic!("Operation was neither 'AND', 'OR' not 'XOR' ({operation:?})")
    }
}

impl LogicGate<'_> {
    fn wire_value(&self, gates: &AllGates<'_>) -> bool {
        if let Some(cached) = self.generated_output.get() {
            return cached;
        }

        let left = gates.lookup(self.left);
        let result = match self.op {
            LogicalOp::And => left && gates.lookup(self.right),
            LogicalOp::Or => left || gates.lookup(self.right),
            LogicalOp::Xor => left != gates.lookup(self.right),
        };

        self.generated_output.set(Some(result));
        result
    }
}

#[derive(Debug)]
struct AllGates<'s> {
    initial_values: FnvHashMap<&'s str, bool>,
    mapping: FnvHashMap<&'s str, LogicGate<'s>>,
}

impl AllGates<'_> {
    fn lookup(&self, gate: &str) -> bool {
        if let Some(gate) = self.mapping.get(gate) {
            gate.wire_value(self)
        } else if let Some(init) = self.initial_values.get(gate) {
            *init
        } else {
            false
        }
    }

    fn get_number(&self) -> u64 {
        let mut result = 0;
        for i in 0..u64::BITS {
            result |= (self.lookup(&format!("z{i:0>2}")) as u64) << i;
        }

        result
    }

    #[allow(unused)]
    fn dump_to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::create(filename)?;
        writeln!(file, "{self}")?;
        Ok(())
    }
}

impl std::fmt::Display for AllGates<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "digraph G {{")?;
        writeln!(f, "   layout=\"fdp\"\n")?;

        for gate in self.initial_values.keys() {
            writeln!(f, "   {gate} [label=\"{gate}\" shape=circle]")?;
        }

        writeln!(f)?;

        for (gate, op) in self.mapping.iter() {
            writeln!(f, "   {gate} [label=\"{gate}\" {}]", op.op)?;
        }

        writeln!(f)?;

        for (res, gate) in self.mapping.iter() {
            writeln!(f, "   {} -> {res}", gate.left)?;
            writeln!(f, "   {} -> {res}", gate.right)?;
        }

        write!(f, "}}")
    }
}

fn parse(input: &str) -> AllGates<'_> {
    let mut lines = input.lines();
    let mut initial = FnvHashMap::default();
    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        let (name, value) = line
            .split_once(':')
            .expect("Could not split initial value on ':'");
        initial.insert(
            name,
            value
                .trim()
                .parse::<u8>()
                .expect("Could not parse value to 0 or 1")
                != 0,
        );
    }

    let mut gates = FnvHashMap::default();
    for line in lines.filter(|line| !line.is_empty()) {
        let (name, gate) = LogicGate::parse_logic_gate(line.trim());
        gates.insert(name, gate);
    }

    AllGates {
        initial_values: initial,
        mapping: gates,
    }
}

#[aoc(day24, part1)]
fn part1(input: &str) -> u64 {
    let gates = parse(input);
    gates.get_number()
}

#[allow(unused)]
#[aoc(day24, part2, hard_coded)]
fn part2_hard_coded(input: &str) -> String {
    // Those were found by manually inspecting the graphviz output of the connections
    const SWAPPED_PAIRS: [(&str, &str); 4] = [
        // Anomaly near x33, y33 and z33
        ("cqm", "vjv"),
        // Anomaly near x25, y25 and z25
        ("z25", "mps"),
        // Anomaly near x19, y19 and z19
        ("z19", "vwp"),
        // Anomaly near x13, y13 and z13
        ("z13", "vcv"),
    ];

    // let gates = parse(input);
    // gates
    // .dump_to_file("input.gv")
    // .expect("Could not write graph to external file");

    let mut swapped: [&str; 8] = SWAPPED_PAIRS
        .into_iter()
        .flat_map(|(a, b)| [a, b])
        .collect_vec()
        .try_into()
        .expect("SWAPPED_PAIRS did not generate 8 values");

    swapped.sort_unstable();
    let mut result = swapped[0].to_owned();
    for el in &swapped[1..] {
        result.push(',');
        result.push_str(el);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const EXAMPLE2: &str = "x00: 1
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
tnw OR pbm -> gnj";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE1), 4);
        assert_eq!(part1(EXAMPLE2), 2024);
    }
}
