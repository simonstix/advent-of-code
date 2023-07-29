use anyhow::bail;
use itertools::Itertools;
use std::iter;
use std::str::FromStr;

const ROCKS: &str = r#"####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##"#;

type Point2 = na::Point2<i64>;
type Vector2 = na::Vector2<i64>;

#[derive(Debug)]
struct Rock {
    /// Tiles occupied by the rock, as offsets from the rock origin point (bottom left)
    positions: Vec<Vector2>,
    #[allow(dead_code)]
    width: usize,
    #[allow(dead_code)]
    height: usize,
}

impl FromStr for Rock {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut positions = vec![];
        let mut max_x = 0;
        let mut max_y = 0;

        // Enumerate reverse (higher y is higher up)
        for (y, line) in s.lines().rev().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                    positions.push(Vector2::new(x as i64, y as i64));
                }
            }
        }

        assert!(!positions.is_empty());
        Ok(Rock {
            positions,
            width: max_x + 1,
            height: max_y + 1,
        })
    }
}

struct Map {
    fallen_rocks: Vec<bool>,
    width: usize,
    current_height: usize,
}

impl Map {
    fn new(width: usize) -> Self {
        Self {
            fallen_rocks: vec![],
            width,
            current_height: 0,
        }
    }

    /// Returns true if the positions is occupied
    fn get(&self, pos: &Point2) -> bool {
        match self.index(pos) {
            Index::Outside => true,
            Index::Above => false,
            Index::Inside(index) => self.fallen_rocks[index],
        }
    }

    fn set(&mut self, pos: &Point2, fill: bool) {
        if fill {
            // Resize array
            let missing_rows = (pos.y - self.current_height as i64 + 1).max(0) as usize;
            if missing_rows > 0 {
                self.fallen_rocks
                    .extend(iter::repeat(false).take(missing_rows * self.width));
                self.current_height += missing_rows;
            }
        }

        match self.index(pos) {
            Index::Outside => {
                panic!("tried setting outside position");
            }
            Index::Above => {
                panic!("vec was not extended far enough");
            }
            Index::Inside(index) => {
                self.fallen_rocks[index] = fill;
            }
        }
    }

    /// Check if a rock can occupy the chosen position
    fn check_rock(&self, pos: &Point2, rock: &Rock) -> bool {
        rock.positions.iter().all(|x| !self.get(&(pos + x)))
    }

    fn set_rock(&mut self, pos: &Point2, rock: &Rock) {
        for offset in &rock.positions {
            self.set(&(pos + offset), true);
        }
    }

    fn index(&self, pos: &Point2) -> Index {
        if pos.x < 0 || pos.x >= self.width as i64 || pos.y < 0 {
            return Index::Outside;
        }

        if pos.y >= self.current_height as i64 {
            return Index::Above;
        }

        Index::Inside((pos.x + pos.y * self.width as i64) as usize)
    }

    fn top_rows(&self, lower: usize, upper: usize) -> &[bool] {
        let lower = lower.min(self.current_height);
        let upper = upper.min(self.current_height);

        let lower = lower * self.width;
        let upper = upper * self.width;
        self.fallen_rocks.get(lower..upper).unwrap()
    }

    #[allow(dead_code)]
    fn to_pretty_string(&self) -> String {
        let mut map = String::new();
        for y in (0..self.current_height).rev() {
            map.push('|');
            for x in 0..self.width {
                if self.get(&Point2::new(x as i64, y as i64)) {
                    map.push('#');
                } else {
                    map.push('.');
                }
            }
            map.push('|');
            map.push('\n');
        }

        map.push('+');
        for _ in 0..self.width {
            map.push('-');
        }
        map.push('+');

        map
    }
}

enum Index {
    Outside,
    Above,
    Inside(usize),
}

#[derive(Copy, Clone)]
enum Movement {
    Left,
    Right,
}

impl Movement {
    fn offset(&self) -> Vector2 {
        match self {
            Movement::Left => Vector2::new(-1, 0),
            Movement::Right => Vector2::new(1, 0),
        }
    }
}

impl TryFrom<char> for Movement {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '<' => Movement::Left,
            '>' => Movement::Right,
            _ => {
                bail!("invalid char")
            }
        })
    }
}

struct FallingRockPattern {
    next_movement: usize,
    next_rock: usize,
    area: Vec<bool>,

    current_height: usize,
    current_rocks: usize,
}

struct FallingRocks<'a> {
    map: Map,
    movements: &'a [Movement],
    next_movement: usize,
    rocks: &'a [Rock],
    next_rock: usize,

    max_fall_height: usize,

    additional_height: usize,
}

impl<'a> FallingRocks<'a> {
    fn new(width: usize, rocks: &'a [Rock], movements: &'a [Movement]) -> Self {
        Self {
            map: Map::new(width),
            movements,
            next_movement: 0,
            rocks,
            next_rock: 0,
            max_fall_height: 0,
            additional_height: 0,
        }
    }

    fn current_pattern(&self, current_rocks: usize) -> FallingRockPattern {
        FallingRockPattern {
            next_movement: self.next_movement,
            next_rock: self.next_rock,
            area: self.current_area().to_vec(),
            current_height: self.current_height(),
            current_rocks,
        }
    }

    fn current_area(&self) -> &[bool] {
        let upper = self.map.current_height + 3;
        let lower = upper - self.max_fall_height;
        self.map.top_rows(lower, upper)
    }

    fn current_height(&self) -> usize {
        self.map.current_height + self.additional_height
    }

    fn drop_n_rocks(&mut self, n: usize) {
        for _ in 0..n {
            self.drop_next_rock();
        }
    }

    fn drop_n_rocks_with_period_search(&mut self, n: usize, warmup: usize) {
        assert!(n > warmup);

        self.drop_n_rocks(warmup);

        let max_fall_height = self.max_fall_height;

        let search_pattern = self.current_pattern(warmup);
        let mut i = warmup;
        while i < n {
            self.drop_next_rock();
            i += 1;

            assert_eq!(max_fall_height, self.max_fall_height);

            if self.next_rock == search_pattern.next_rock
                && self.next_movement == search_pattern.next_movement
                && self.current_area() == search_pattern.area
            {
                // Pattern is fully aligned, skip to the end now!
                let rock_diff = i - search_pattern.current_rocks;
                let height_diff = self.current_height() - search_pattern.current_height;

                let missing_rocks = n - i - 1;
                let loops = missing_rocks / rock_diff;

                i += loops * rock_diff;
                self.additional_height += loops * height_diff;
                break;
            }
        }

        self.drop_n_rocks(n - i);
    }

    fn next_move(&mut self) -> Movement {
        let movement = self.movements[self.next_movement];
        self.next_movement = (self.next_movement + 1) % self.movements.len();
        movement
    }

    fn drop_next_rock(&mut self) {
        let rock = &self.rocks[self.next_rock];
        self.next_rock = (self.next_rock + 1) % self.rocks.len();

        let mut fall_height = 0;
        let mut pos = Point2::new(2, self.map.current_height as i64 + 3);
        loop {
            // Wind push
            let movement = self.next_move();
            let new_pos = pos + movement.offset();
            if self.map.check_rock(&new_pos, rock) {
                pos = new_pos;
            }

            // Move down
            let new_pos = pos + Vector2::new(0, -1);
            if self.map.check_rock(&new_pos, rock) {
                fall_height += 1;
                pos = new_pos;
            } else {
                break;
            }
        }

        self.max_fall_height = self.max_fall_height.max(fall_height);

        // Fixate rock in map
        self.map.set_rock(&pos, rock);
    }
}

pub fn day17(content: String) {
    println!();
    println!("==== Day 17 ====");
    let rocks = ROCKS
        .split("\n\n")
        .map(|x| x.parse::<Rock>().unwrap())
        .collect_vec();
    let movements = content
        .chars()
        .map(|x| Movement::try_from(x).unwrap())
        .collect_vec();

    println!("Part 1");
    let mut falling_rocks = FallingRocks::new(7, &rocks, &movements);
    falling_rocks.drop_n_rocks(2022);
    println!("Height: {}", falling_rocks.current_height());

    println!();
    println!("Part 2");
    let mut falling_rocks = FallingRocks::new(7, &rocks, &movements);
    falling_rocks.drop_n_rocks_with_period_search(1000000000000, 1000000);
    println!("Height: {}", falling_rocks.current_height());
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    const EXAMPLE: &'static str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;

    #[test]
    fn test_part_1() {
        let rocks = ROCKS
            .split("\n\n")
            .map(|x| x.parse::<Rock>().unwrap())
            .collect_vec();
        let movements = EXAMPLE
            .chars()
            .map(|x| Movement::try_from(x).unwrap())
            .collect_vec();
        let mut falling_rocks = FallingRocks::new(7, &rocks, &movements);
        falling_rocks.drop_n_rocks(2022);
        assert_eq!(falling_rocks.current_height(), 3068);
    }

    #[test]
    fn test_part_2() {
        let rocks = ROCKS
            .split("\n\n")
            .map(|x| x.parse::<Rock>().unwrap())
            .collect_vec();
        let movements = EXAMPLE
            .chars()
            .map(|x| Movement::try_from(x).unwrap())
            .collect_vec();
        let mut falling_rocks = FallingRocks::new(7, &rocks, &movements);
        falling_rocks.drop_n_rocks_with_period_search(1000000000000, 1000000);
        assert_eq!(falling_rocks.current_height(), 1514285714288);
    }
}
