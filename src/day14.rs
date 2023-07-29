use anyhow::Context;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

type Point2 = na::Point2<i64>;
type Vector2 = na::Vector2<i64>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Air,
    Wall,
    Sand,
}

impl Tile {
    #[allow(dead_code)]
    fn as_char(&self) -> char {
        match self {
            Tile::Air => '.',
            Tile::Wall => '#',
            Tile::Sand => 'o',
        }
    }
}

struct Map {
    tiles: HashMap<Point2, Tile>,
    max_y: i64,
    has_floor: bool,
}

impl Map {
    fn from_paths(s: &str, has_floor: bool) -> Self {
        let mut tiles = HashMap::new();

        let mut max_y = 0;
        for path in s.lines() {
            let path = path.parse::<Path>().unwrap();
            for point in path.iter() {
                max_y = max_y.max(point.y);
                tiles.insert(point, Tile::Wall);
            }
        }

        Self {
            tiles,
            has_floor,
            max_y,
        }
    }

    fn add_sand(&mut self, mut pos: Point2, max_settle_ticks: usize) -> bool {
        if self.get(pos) != Tile::Air {
            return false;
        }

        self.set(pos, Tile::Sand);

        const DIRS: [Vector2; 3] = [Vector2::new(0, 1), Vector2::new(-1, 1), Vector2::new(1, 1)];
        'update: for _ in 0..max_settle_ticks {
            for dir in DIRS {
                let target = pos + dir;
                if self.get(target) == Tile::Air {
                    self.set(pos, Tile::Air);
                    self.set(target, Tile::Sand);
                    pos = target;
                    continue 'update;
                }
            }

            // No position was found
            return true;
        }

        // Ran out of time
        false
    }

    fn fill_sand(&mut self, pos: Point2, max_settle_ticks: usize) -> usize {
        let mut counter = 0;
        while self.add_sand(pos, max_settle_ticks) {
            counter += 1;
        }
        counter
    }

    fn get(&self, pos: Point2) -> Tile {
        if self.has_floor && pos.y >= self.max_y + 2 {
            return Tile::Wall;
        }

        self.tiles.get(&pos).copied().unwrap_or(Tile::Air)
    }

    fn set(&mut self, pos: Point2, tile: Tile) -> Tile {
        let old = self.get(pos);
        self.tiles.insert(pos, tile);
        old
    }

    #[allow(dead_code)]
    fn print_map(&self, x_start: i64, x_end: i64, y_start: i64, y_end: i64) {
        let mut s = String::new();
        for y in y_start..=y_end {
            for x in x_start..=x_end {
                s.push(self.get(Point2::new(x, y)).as_char());
            }
            s.push('\n');
        }
        println!("{}", s);
    }
}

struct Path {
    path: Vec<Point2>,
}

impl Path {
    fn iter(&self) -> PathIter {
        PathIter {
            path: self,
            target_index: 0,
            pos: *self.path.first().unwrap(),
        }
    }

    fn get(&self, index: usize) -> Option<Point2> {
        self.path.get(index).copied()
    }
}

impl FromStr for Path {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = s.split(" -> ").map(parse_point).try_collect()?;
        Ok(Self { path })
    }
}

fn parse_point(point: &str) -> anyhow::Result<Point2> {
    let (x, y) = point
        .split(',')
        .collect_tuple()
        .context("point must have 2 coordinates")?;
    let x = x.parse::<i64>().context("could not parse x")?;
    let y = y.parse::<i64>().context("could not parse y")?;
    Ok(Point2::new(x, y))
}

struct PathIter<'a> {
    path: &'a Path,
    target_index: usize,
    pos: Point2,
}

impl<'a> Iterator for PathIter<'a> {
    type Item = Point2;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.pos;

        let target = self.path.get(self.target_index)?;

        if self.pos == target {
            self.target_index += 1;
        }

        if let Some(target) = self.path.get(self.target_index) {
            let dir = integer_normalize(target - self.pos);
            self.pos += dir;
        }

        Some(result)
    }
}

fn integer_normalize(mut vector: Vector2) -> Vector2 {
    if vector.x != 0 && vector.y != 0 {
        panic!();
    }

    vector.x = vector.x.signum();
    vector.y = vector.y.signum();
    vector
}

pub fn day14(content: String) {
    println!();
    println!("==== Day 14 ====");

    println!("Part 1");
    let mut map = Map::from_paths(&content, false);
    println!(
        "Fitting grains of sand: {}",
        map.fill_sand(Point2::new(500, 0), 200)
    );

    println!();
    println!("Part 2");
    let mut _map = Map::from_paths(&content, true);
    println!("Part 2 skipped for performance");
    // println!(
    //     "Fitting grains of sand: {}",
    //     map.fill_sand(Point2::new(500, 0), 400)
    // );
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

    #[test]
    fn test_part_1() {
        let mut map = Map::from_paths(EXAMPLE, false);
        assert_eq!(map.fill_sand(Point2::new(500, 0), 100), 24);
    }

    #[test]
    fn test_part_2() {
        let mut map = Map::from_paths(EXAMPLE, true);
        assert_eq!(map.fill_sand(Point2::new(500, 0), 100), 93);
    }
}
