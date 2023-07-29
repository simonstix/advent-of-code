use anyhow::{anyhow, bail, Context};
use itertools::Itertools;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::str::FromStr;

trait Watcher {
    fn watch_step(&mut self, vm: &VMState);
}

struct VMState {
    program_counter: usize,
    register: i64,
}

struct VM {
    state: VMState,
    watchers: Vec<Box<dyn Watcher>>,
}

impl VM {
    fn new() -> Self {
        Self {
            state: VMState {
                program_counter: 1,
                register: 1,
            },
            watchers: vec![],
        }
    }

    fn run(&mut self, program: Vec<Instruction>) {
        for instruction in program {
            self.execute(instruction);
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Add(num) => {
                self.step();
                self.step();
                self.state.register += num;
            }
            Instruction::Noop => {
                self.step();
            }
        }
    }

    fn step(&mut self) {
        for watcher in self.watchers.iter_mut() {
            watcher.watch_step(&self.state);
        }

        self.state.program_counter += 1;
    }
}

enum Instruction {
    Add(i64),
    Noop,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect_vec();
        let op = *parts.first().ok_or_else(|| anyhow!("no operation"))?;
        let instruction = match op {
            "addx" => Self::Add(
                parts
                    .get(1)
                    .ok_or_else(|| anyhow!("addx expected argument i64"))
                    .and_then(|x| x.parse::<i64>().context("addx expected argument i64"))?,
            ),
            "noop" => Self::Noop,
            _ => bail!("unexpected op"),
        };
        Ok(instruction)
    }
}

struct SignalStrengthWatcher {
    signal: Rc<Cell<i64>>,
}

impl SignalStrengthWatcher {
    fn new(signal: Rc<Cell<i64>>) -> Self {
        Self { signal }
    }
}

impl Watcher for SignalStrengthWatcher {
    fn watch_step(&mut self, vm: &VMState) {
        if vm.program_counter < 20 || (vm.program_counter - 20) % 40 != 0 {
            return;
        }

        let mut total = self.signal.get();
        let current_signal = vm.program_counter as i64 * vm.register;
        total += current_signal;
        // println!(
        //     "Cycle {}, register: {}, signal: {}, total: {}",
        //     vm.program_counter, vm.register, current_signal, total
        // );
        self.signal.set(total);
    }
}

struct ScreenWatcher {
    screen: Rc<RefCell<String>>,
    x: i64,
}

impl ScreenWatcher {
    fn new(screen: Rc<RefCell<String>>) -> Self {
        Self { screen, x: 0 }
    }
}

impl Watcher for ScreenWatcher {
    fn watch_step(&mut self, vm: &VMState) {
        if vm.program_counter != 1 && vm.program_counter % 40 == 1 {
            self.screen.borrow_mut().push('\n');
            self.x = 0;
        }

        if (self.x - vm.register).abs() <= 1 {
            self.screen.borrow_mut().push('#');
        } else {
            self.screen.borrow_mut().push('.');
        }

        // println!(
        //     "Cycle {: >3}, register: {: >2}, line: {}",
        //     vm.program_counter,
        //     vm.register,
        //     self.screen.borrow().lines().last().unwrap()
        // );

        self.x += 1;
    }
}

pub fn day10(content: String) {
    println!();
    println!("==== Day 10 ====");
    let program = content
        .lines()
        .map(|x| x.parse::<Instruction>().unwrap())
        .collect_vec();
    let mut vm = VM::new();

    let signal_strength = Rc::new(Cell::new(0));
    let signal_strength_watcher = SignalStrengthWatcher::new(signal_strength.clone());
    vm.watchers.push(Box::new(signal_strength_watcher));

    let screen = Rc::new(RefCell::new(String::new()));
    let screen_watcher = ScreenWatcher::new(screen.clone());
    vm.watchers.push(Box::new(screen_watcher));

    vm.run(program);

    println!("Part 1");
    println!("Total signal strength: {}", signal_strength.get());

    println!();
    println!("Part 2");
    println!("Screen:");
    println!("{}", screen.borrow());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Deref;

    const EXAMPLE: &'static str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

    #[test]
    fn test_part_1() {
        let program = EXAMPLE
            .lines()
            .map(|x| x.parse::<Instruction>().unwrap())
            .collect_vec();

        let signal_strength = Rc::new(Cell::new(0));
        let signal_strength_watcher = SignalStrengthWatcher::new(signal_strength.clone());
        let mut vm = VM::new();
        vm.watchers.push(Box::new(signal_strength_watcher));

        vm.run(program);

        assert_eq!(signal_strength.get(), 13140);
    }

    #[test]
    fn test_part_2() {
        let program = EXAMPLE
            .lines()
            .map(|x| x.parse::<Instruction>().unwrap())
            .collect_vec();

        let screen = Rc::new(RefCell::new(String::new()));
        let screen_watcher = ScreenWatcher::new(screen.clone());
        let mut vm = VM::new();
        vm.watchers.push(Box::new(screen_watcher));

        vm.run(program);

        assert_eq!(
            screen.borrow().deref(),
            r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#
        );
    }
}
