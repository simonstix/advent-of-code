use anyhow::bail;
use itertools::Itertools;
use na::Vector2;
use std::collections::HashSet;
use std::str::FromStr;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn to_vector(&self) -> Vector2<i32> {
        match self {
            Direction::Up => Vector2::new(0, 1),
            Direction::Right => Vector2::new(1, 0),
            Direction::Down => Vector2::new(0, -1),
            Direction::Left => Vector2::new(-1, 0),
        }
    }
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            dir => bail!("unsupported direction {}", dir),
        })
    }
}

struct Command {
    direction: Direction,
    steps: usize,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, steps) = s.split_whitespace().collect_tuple().unwrap();
        let direction = direction.parse::<Direction>().unwrap();
        let steps = steps.parse::<usize>().unwrap();
        Ok(Self { direction, steps })
    }
}

struct RopeFollow {
    visited: HashSet<Vector2<i32>>,
    rope: Vec<Vector2<i32>>,
}

impl RopeFollow {
    fn new(rope_length: usize) -> Self {
        let start = Vector2::new(0, 0);
        let mut visited = HashSet::new();
        visited.insert(start);
        Self {
            visited,
            rope: vec![start; rope_length],
        }
    }

    fn execute_commands(&mut self, commands: &[Command]) {
        for command in commands {
            for _ in 0..command.steps {
                let dir = command.direction.to_vector();
                self.step(&dir);
            }
        }
    }

    fn step(&mut self, dir: &Vector2<i32>) {
        *self.rope.first_mut().unwrap() += dir;
        self.update_tail();
    }

    fn update_tail(&mut self) {
        let mut head = *self.rope.first().unwrap();
        for tail in self.rope.iter_mut().skip(1) {
            let distance = head - *tail;
            let distance_clamped = distance.map(|x| x.clamp(-1, 1));

            // The tail is adjacent to the head, it doesn't have to move
            if distance == distance_clamped {
                return;
            }

            *tail += distance_clamped;

            head = *tail;
        }

        self.visited.insert(*self.rope.last().unwrap());
    }

    fn count_visited(&self) -> usize {
        self.visited.len()
    }
}

pub fn day9(content: String) {
    println!();
    println!("==== Day 9 ====");
    let commands = content
        .lines()
        .map(|x| x.parse::<Command>().unwrap())
        .collect_vec();

    println!("Part 1");
    let mut rope = RopeFollow::new(2);
    rope.execute_commands(&commands);
    println!("Tail visited positions: {}", rope.count_visited());

    println!("Part 2");
    let mut rope = RopeFollow::new(10);
    rope.execute_commands(&commands);
    println!("Tail visited positions: {}", rope.count_visited());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

    const EXAMPLE_2: &'static str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

    #[test]
    fn test_part_1() {
        let commands = EXAMPLE
            .lines()
            .map(|x| x.parse::<Command>().unwrap())
            .collect_vec();

        let mut rope = RopeFollow::new(2);
        rope.execute_commands(&commands);

        assert_eq!(rope.count_visited(), 13);
    }

    #[test]
    fn test_part_2_simple() {
        let commands = EXAMPLE
            .lines()
            .map(|x| x.parse::<Command>().unwrap())
            .collect_vec();

        let mut rope = RopeFollow::new(10);
        rope.execute_commands(&commands);

        assert_eq!(rope.count_visited(), 1);
    }

    #[test]
    fn test_part_2_large() {
        let commands = EXAMPLE_2
            .lines()
            .map(|x| x.parse::<Command>().unwrap())
            .collect_vec();

        let mut rope = RopeFollow::new(10);
        rope.execute_commands(&commands);

        assert_eq!(rope.count_visited(), 36);
    }
}
