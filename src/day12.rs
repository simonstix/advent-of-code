use anyhow::{bail, Context};
use na::Vector2;
use pathfinding::prelude::astar;
use std::str::FromStr;

type Point2 = na::Point2<i32>;

enum Tile {
    Start,
    Target,
    Height(u32),
}

impl Tile {
    fn is_target(&self) -> bool {
        matches!(self, Tile::Target)
    }
    fn elevation(&self) -> u32 {
        match self {
            Tile::Start => 0,
            Tile::Target => 'z'.to_digit(36).unwrap() - 'a'.to_digit(36).unwrap(),
            Tile::Height(height) => *height,
        }
    }
}

struct Map {
    tiles: Vec<Tile>,
    width: usize,
    length: usize,
    start_pos: Point2,
    target_pos: Point2,
}

impl Map {
    fn neighbors(&self, pos: &Point2) -> Vec<Point2> {
        let Some(height) = self.height(pos) else {
            return vec![];
        };
        [
            pos + Vector2::new(1, 0),
            pos + Vector2::new(-1, 0),
            pos + Vector2::new(0, 1),
            pos + Vector2::new(0, -1),
        ]
        .into_iter()
        .filter(|pos| self.height(pos).unwrap_or(99) <= height + 1)
        .collect()
    }

    fn get(&self, pos: &Point2) -> Option<&Tile> {
        let index = self.index(pos)?;
        self.tiles.get(index)
    }

    fn height(&self, pos: &Point2) -> Option<u32> {
        let index = self.index(pos)?;
        self.tiles.get(index).map(|x| x.elevation())
    }

    fn index(&self, pos: &Point2) -> Option<usize> {
        if pos.x < 0 || pos.x >= self.width as i32 || pos.y < 0 || pos.y >= self.length as i32 {
            return None;
        }
        Some((pos.x + pos.y * self.width as i32) as usize)
    }

    fn index_to_pos(&self, index: usize) -> Option<Point2> {
        if index >= self.tiles.len() {
            return None;
        }

        Some(Point2::new(
            (index % self.width) as i32,
            (index / self.width) as i32,
        ))
    }

    fn shortest_path_length_from_start(&self) -> usize {
        self.shortest_path_length(&self.start_pos).unwrap()
    }

    fn shortest_path_length(&self, pos: &Point2) -> Option<usize> {
        let (path, _) = astar(
            pos,
            |pos| self.neighbors(pos).into_iter().map(|x| (x, 1 /* cost */)),
            |pos| self.target_pos.x.abs_diff(pos.x) + self.target_pos.y.abs_diff(pos.y),
            |pos| self.get(pos).unwrap().is_target(),
        )?;

        Some(path.len() - 1)
    }

    #[allow(dead_code)]
    fn find_closest_start_point(&self) -> usize {
        self.tiles
            .iter()
            .enumerate()
            .filter(|(_, x)| x.elevation() == 0)
            .map(|(index, _)| self.index_to_pos(index).unwrap())
            .filter_map(|pos| self.shortest_path_length(&pos))
            .min()
            .unwrap()
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let min_char_value = 'a'.to_digit(36).unwrap();

        let mut start_pos = Point2::origin();
        let mut target_pos = Point2::origin();
        let mut tiles = vec![];
        let mut height = 0;
        let mut width = 0;
        for (y, line) in s.lines().enumerate() {
            height += 1;
            if width == 0 {
                width = line.len();
            } else if width != line.len() {
                bail!("different line lengths");
            }

            for (x, tile) in line.chars().enumerate() {
                let tile = match tile {
                    'S' => {
                        start_pos = Point2::new(x as i32, y as i32);
                        Tile::Start
                    }
                    'E' => {
                        target_pos = Point2::new(x as i32, y as i32);
                        Tile::Target
                    }
                    height => Tile::Height(
                        height.to_digit(36).context("invalid character")? - min_char_value,
                    ),
                };
                tiles.push(tile);
            }
        }

        Ok(Map {
            tiles,
            length: height,
            width,
            start_pos,
            target_pos,
        })
    }
}

pub fn day12(content: String) {
    println!();
    println!("==== Day 12 ====");
    let map = content.parse::<Map>().unwrap();

    println!("Part 1");
    println!(
        "Shortest path length: {}",
        map.shortest_path_length_from_start()
    );

    println!();
    println!("Part 2");
    println!("Ignoring part two to safe time");
    // println!("Shortest path length: {}", map.find_closest_start_point());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

    #[test]
    fn test_part_1() {
        let map = EXAMPLE.parse::<Map>().unwrap();
        assert_eq!(map.shortest_path_length_from_start(), 31);
    }

    #[test]
    fn test_part_2() {
        let map = EXAMPLE.parse::<Map>().unwrap();
        assert_eq!(map.find_closest_start_point(), 29);
    }
}
