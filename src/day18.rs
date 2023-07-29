use anyhow::{anyhow, Context};
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list1;
use nom::{Finish, IResult};
use std::str::FromStr;

type Point3 = na::Point3<i64>;
type Vector3 = na::Vector3<i64>;

const NEIGHBORS: &[Vector3] = &[
    Vector3::new(-1, 0, 0),
    Vector3::new(1, 0, 0),
    Vector3::new(0, -1, 0),
    Vector3::new(0, 1, 0),
    Vector3::new(0, 0, -1),
    Vector3::new(0, 0, 1),
];

struct Grid {
    tiles: Vec<bool>,
    outside: Vec<bool>,
    width: usize,
    height: usize,
    length: usize,
}

impl Grid {
    fn new(width: usize, height: usize, length: usize) -> Self {
        Self {
            tiles: vec![false; width * height * length],
            outside: vec![false; width * height * length],
            width,
            height,
            length,
        }
    }

    fn get_filled(&self, pos: &Point3) -> bool {
        let Some(index) = self.index(pos) else {
            return false;
        };

        *self.tiles.get(index).expect("index out of range")
    }

    fn get_outside(&self, pos: &Point3) -> bool {
        let Some(index) = self.index(pos) else {
            return true;
        };

        *self.outside.get(index).expect("index out of range")
    }

    fn set_filled(&mut self, pos: &Point3, is_filled: bool) {
        let index = self
            .index(pos)
            .unwrap_or_else(|| panic!("tried to set out of range: {}", pos));
        *self.tiles.get_mut(index).expect("index out of range") = is_filled;
    }
    fn set_outside(&mut self, pos: &Point3, is_outside: bool) {
        let index = self
            .index(pos)
            .unwrap_or_else(|| panic!("tried to set out of range: {}", pos));
        *self.outside.get_mut(index).expect("index out of range") = is_outside;
    }

    fn index(&self, pos: &Point3) -> Option<usize> {
        if pos.x < 0
            || pos.x >= self.width as i64
            || pos.y < 0
            || pos.y >= self.height as i64
            || pos.z < 0
            || pos.z >= self.length as i64
        {
            return None;
        }

        Some(
            (pos.x + pos.y * self.width as i64 + pos.z * self.width as i64 * self.height as i64)
                .try_into()
                .expect("invalid index"),
        )
    }

    fn count_open_sides(&self) -> usize {
        let mut sides = 0;
        for z in 0..self.length {
            for y in 0..self.height {
                for x in 0..self.width {
                    let pos = Point3::new(x as i64, y as i64, z as i64);

                    // Position is not occupied
                    if !self.get_filled(&pos) {
                        continue;
                    }

                    sides += self.count_empty_neighbors(&pos);
                }
            }
        }
        sides
    }

    fn count_empty_neighbors(&self, pos: &Point3) -> usize {
        let mut count = 0;
        for offset in NEIGHBORS {
            let offset_pos = pos + offset;

            // Is empty
            if !self.get_filled(&offset_pos) {
                count += 1;
            }
        }
        count
    }

    fn count_outside_sides(&self) -> usize {
        let mut sides = 0;
        for z in 0..self.length {
            for y in 0..self.height {
                for x in 0..self.width {
                    let pos = Point3::new(x as i64, y as i64, z as i64);

                    // Position is not occupied
                    if !self.get_filled(&pos) {
                        continue;
                    }

                    sides += self.count_outside_neighbors(&pos);
                }
            }
        }
        sides
    }

    fn count_outside_neighbors(&self, pos: &Point3) -> usize {
        let mut count = 0;
        for offset in NEIGHBORS {
            let offset_pos = pos + offset;

            // Is outside
            if self.get_outside(&offset_pos) {
                count += 1;
            }
        }
        count
    }

    fn flood_fill_outside(&mut self, start_pos: &Point3) {
        let mut stack = vec![*start_pos];

        while let Some(pos) = stack.pop() {
            // Is in range
            if self.index(&pos).is_none() {
                continue;
            }

            // Check if pos is filled
            if self.get_filled(&pos) {
                continue;
            }

            // Was already filled
            if self.get_outside(&pos) {
                continue;
            }

            self.set_outside(&pos, true);

            // Add all neighbors
            for neighbor in NEIGHBORS {
                stack.push(pos + neighbor);
            }
        }
    }

    #[allow(dead_code)]
    fn print_filled_sides(&self) {
        for z in 0..self.length {
            for y in 0..self.height {
                for x in 0..self.width {
                    let pos = Point3::new(x as i64, y as i64, z as i64);
                    if self.get_filled(&pos) {
                        println!("{}", pos);
                    }
                }
            }
        }
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (_, positions) = separated_list1(tag("\n"), parse_pos)(input)
            .finish()
            .map_err(|_| anyhow!("could not parse list"))?;
        let max_x: usize = positions
            .iter()
            .map(|p| p.x)
            .max()
            .context("no points in grid")?
            .try_into()?;
        let max_y: usize = positions
            .iter()
            .map(|p| p.y)
            .max()
            .context("no points in grid")?
            .try_into()?;
        let max_z: usize = positions
            .iter()
            .map(|p| p.z)
            .max()
            .context("no points in grid")?
            .try_into()?;
        let width = max_x + 2;
        let height = max_y + 2;
        let length = max_z + 2;

        let mut grid = Grid::new(width, height, length);

        for position in positions {
            grid.set_filled(&position, true);
        }

        grid.flood_fill_outside(&Point3::new(0, 0, 0));

        Ok(grid)
    }
}

fn parse_pos(input: &str) -> IResult<&str, Point3> {
    let (input, x) = complete::i64(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = complete::i64(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, z) = complete::i64(input)?;
    Ok((input, Point3::new(x, y, z)))
}

pub fn day18(content: String) {
    println!();
    println!("==== Day 18 ====");
    let grid = content.parse::<Grid>().unwrap();

    println!("Part 1");
    println!("Sides: {}", grid.count_open_sides());

    println!();
    println!("Part 2");
    println!("Sides: {}", grid.count_outside_sides());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#;

    #[test]
    fn test_single_cube() {
        let mut grid = Grid::new(4, 4, 4);
        grid.set_filled(&Point3::new(2, 2, 2), true);
        assert_eq!(grid.count_open_sides(), 6);
    }

    #[test]
    fn test_two_cubes() {
        let mut grid = Grid::new(4, 4, 4);
        grid.set_filled(&Point3::new(2, 2, 2), true);
        grid.set_filled(&Point3::new(3, 2, 2), true);
        assert_eq!(grid.count_open_sides(), 10);
    }

    #[test]
    fn test_part_1() {
        let grid = EXAMPLE.parse::<Grid>().unwrap();
        assert_eq!(grid.count_open_sides(), 64);
    }

    #[test]
    fn test_part_2() {
        let grid = EXAMPLE.parse::<Grid>().unwrap();
        assert_eq!(grid.count_outside_sides(), 58);
    }
}
