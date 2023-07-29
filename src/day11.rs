use anyhow::{bail, Context};
use itertools::Itertools;
use std::str::FromStr;

type Item = u64;

#[derive(Debug, Clone)]
enum Operand {
    Value(Item),
    Old,
}

impl Operand {
    fn value(&self, old: &Item) -> Item {
        match self {
            Operand::Value(value) => *value,
            Operand::Old => *old,
        }
    }
}

impl FromStr for Operand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = if s == "old" {
            Self::Old
        } else {
            Self::Value(s.parse::<Item>().context("invalid operand value")?)
        };
        Ok(value)
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mult,
}

impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op = match s {
            "+" => Self::Add,
            "*" => Self::Mult,
            _ => bail!("unsupported op"),
        };
        Ok(op)
    }
}

#[derive(Clone)]
struct Operation {
    left: Operand,
    op: Op,
    right: Operand,
}

impl Operation {
    fn calculate(&self, old: &Item, ring: &Item) -> Item {
        let left = self.left.value(old);
        let right = self.right.value(old);

        let result = match self.op {
            Op::Add => left + right,
            Op::Mult => left * right,
        };

        result % ring
    }
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, op, right): (&str, &str, &str) = s
            .split_whitespace()
            .collect_tuple()
            .context("invalid operation")?;
        Ok(Self {
            left: left.parse::<Operand>()?,
            op: op.parse::<Op>()?,
            right: right.parse::<Operand>()?,
        })
    }
}

#[derive(Clone)]
struct Test {
    divisor: Item,
    if_true: usize,
    if_false: usize,
}

impl Test {
    fn target(&self, value: &Item) -> usize {
        if value % self.divisor == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[derive(Default, Clone)]
struct MonkeyStats {
    inspections: usize,
}

#[derive(Clone)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    test: Test,
    stats: MonkeyStats,
}

impl Monkey {
    fn handle_items(&mut self, with_relief: bool, ring: &Item) -> Vec<(Item, usize)> {
        let mut throws = vec![];
        for mut item in self.items.drain(..) {
            // Monkey inspects
            item = self.operation.calculate(&item, ring);
            self.stats.inspections += 1;

            // Worry drains
            if with_relief {
                item /= 3;
            }

            // Test
            let target = self.test.target(&item);

            throws.push((item, target));
        }

        throws
    }
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let _monkey = lines.next().context("missing line: monkey");

        let items = lines.next().context("missing line: starting items")?;
        let items = remove_prefix(items, "Starting items: ")?;
        let items = items
            .split(", ")
            .map(|item| item.parse::<Item>().context("could not parse item"))
            .collect::<anyhow::Result<Vec<Item>>>()?;

        let operation = lines.next().context("missing line: operation")?;
        let operation = remove_prefix(operation, "Operation: new = ")?.parse::<Operation>()?;

        let divisor = lines.next().context("missing line: test")?;
        let divisor = remove_prefix(divisor, "Test: divisible by ")?.parse::<Item>()?;
        let if_true = lines.next().context("missing line: test if true")?;
        let if_true = remove_prefix(if_true, "If true: throw to monkey ")?.parse::<usize>()?;
        let if_false = lines.next().context("missing line: test if false")?;
        let if_false = remove_prefix(if_false, "If false: throw to monkey ")?.parse::<usize>()?;
        let test = Test {
            divisor,
            if_true,
            if_false,
        };

        if lines.next().is_some() {
            bail!("Unexpected data");
        }

        Ok(Monkey {
            items,
            operation,
            test,
            stats: Default::default(),
        })
    }
}

fn remove_prefix<'a>(line: &'a str, prefix: &'static str) -> anyhow::Result<&'a str> {
    let line = line.trim();
    if !line.starts_with(prefix) {
        bail!("unexpected prefix '{}'", prefix);
    }
    Ok(&line[prefix.len()..])
}

#[derive(Clone)]
struct MonkeyGroup {
    monkeys: Vec<Monkey>,
    ring: Item,
}

impl MonkeyGroup {
    fn new(monkeys: Vec<Monkey>) -> Self {
        let ring = monkeys.iter().map(|x| x.test.divisor).product();
        Self { monkeys, ring }
    }

    fn n_rounds(&mut self, rounds: usize, with_relief: bool) {
        for _ in 0..rounds {
            self.round(with_relief);
        }
    }

    fn round(&mut self, with_relief: bool) {
        for i in 0..self.monkeys.len() {
            let monkey = self.monkeys.get_mut(i).unwrap();
            for (item, target) in monkey.handle_items(with_relief, &self.ring) {
                self.monkeys
                    .get_mut(target)
                    .expect("unexpected monkey")
                    .items
                    .push(item);
            }
        }
    }

    fn monkey_business(&self) -> usize {
        self.monkeys
            .iter()
            .map(|x| x.stats.inspections)
            .sorted()
            .rev()
            .take(2)
            .product()
    }
}

impl FromStr for MonkeyGroup {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let monkeys = s
            .split("\n\n")
            .map(|monkey| monkey.parse::<Monkey>().context("could not parse monkey"))
            .collect::<anyhow::Result<Vec<Monkey>>>()?;
        Ok(Self::new(monkeys))
    }
}

pub fn day11(content: String) {
    println!();
    println!("==== Day 11 ====");
    let base_monkeys = content.parse::<MonkeyGroup>().unwrap();

    println!("Part 1");
    let mut monkeys = base_monkeys.clone();
    monkeys.n_rounds(20, true);
    println!("Monkey business: {}", monkeys.monkey_business());

    println!();
    println!("Part 2");
    let mut monkeys = base_monkeys;
    monkeys.n_rounds(10000, false);
    println!("Monkey business: {}", monkeys.monkey_business());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;

    #[test]
    fn test_part_1() {
        let mut monkeys = EXAMPLE.parse::<MonkeyGroup>().unwrap();
        monkeys.n_rounds(20, true);
        assert_eq!(monkeys.monkey_business(), 10605);
    }

    #[test]
    fn test_part_2() {
        let mut monkeys = EXAMPLE.parse::<MonkeyGroup>().unwrap();
        monkeys.n_rounds(10000, false);
        assert_eq!(monkeys.monkey_business(), 2713310158);
    }
}
