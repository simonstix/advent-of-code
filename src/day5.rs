use itertools::Itertools;
use std::iter;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct SupplyStacks {
    stacks: Vec<Vec<char>>,
}

impl SupplyStacks {
    fn pop(&mut self, index: usize) -> Option<char> {
        self.stacks.get_mut(index).unwrap().pop()
    }

    fn push(&mut self, index: usize, cargo: char) {
        self.stacks.get_mut(index).unwrap().push(cargo);
    }

    fn top(&self) -> String {
        self.stacks.iter().filter_map(|x| x.last()).collect()
    }
}

impl FromStr for SupplyStacks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().collect_vec();
        let crate_positions = lines
            .pop()
            .unwrap()
            .chars()
            .enumerate()
            .filter(|(_, c)| !c.is_whitespace())
            .map(|(x, _)| x)
            .collect_vec();

        let mut stacks = iter::repeat(vec![])
            .take(crate_positions.len())
            .collect_vec();
        while let Some(level) = lines.pop() {
            for (&index, stack) in crate_positions.iter().zip(stacks.iter_mut()) {
                let c = level.chars().nth(index).unwrap();
                if !c.is_whitespace() {
                    stack.push(c);
                }
            }
        }
        Ok(SupplyStacks { stacks })
    }
}

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    pub fn execute_single_crate(&self, stack: &mut SupplyStacks) {
        for _ in 0..self.count {
            let c = stack.pop(self.from).unwrap();
            stack.push(self.to, c);
        }
    }

    pub fn execute_multi_crate(&self, stack: &mut SupplyStacks) {
        let mut temp = vec![];
        for _ in 0..self.count {
            let c = stack.pop(self.from).unwrap();
            temp.push(c);
        }

        while let Some(c) = temp.pop() {
            stack.push(self.to, c);
        }
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split_whitespace();

        // Move
        let _ = line.next().unwrap();
        let count = line.next().unwrap().parse::<usize>()?;

        // From
        let _ = line.next().unwrap();
        let from = line.next().unwrap().parse::<usize>()?;
        let from = from - 1;

        // To
        let _ = line.next().unwrap();
        let to = line.next().unwrap().parse::<usize>()?;
        let to = to - 1;

        Ok(Instruction { count, from, to })
    }
}

pub fn day5(content: String) {
    println!();
    println!("==== Day 5 ====");
    let (stacks, instructions) = content.split("\n\n").collect_tuple().unwrap();

    let original_stacks = stacks.parse::<SupplyStacks>().unwrap();
    let instructions = instructions
        .lines()
        .map(|x| x.parse::<Instruction>().unwrap())
        .collect_vec();

    println!("Part 1");
    let mut stacks = original_stacks.clone();
    for instruction in &instructions {
        instruction.execute_single_crate(&mut stacks);
    }
    println!("Top: {}", stacks.top());

    println!("Part 2");
    let mut stacks = original_stacks;
    for instruction in &instructions {
        instruction.execute_multi_crate(&mut stacks);
    }
    println!("Top: {}", stacks.top());
}
