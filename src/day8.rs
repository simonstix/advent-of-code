use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

#[derive(Debug)]
struct VisibilityMap {
    visible: Vec<bool>,
    width: usize,
    _height: usize,
}

impl VisibilityMap {
    fn count_visible(&self) -> usize {
        self.visible.iter().filter(|x| **x).count()
    }
}

impl Display for VisibilityMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.visible.chunks(self.width) {
            for c in line.iter().map(|&x| if x { 'V' } else { ' ' }) {
                f.write_char(c).unwrap();
            }

            f.write_char('\n').unwrap();
        }
        Ok(())
    }
}

enum LookDir {
    Up,
    Right,
    Down,
    Left,
}

struct TreeGrid {
    trees: Vec<u8>,
    width: usize,
    height: usize,
}

impl TreeGrid {
    fn tree(&self, x: usize, y: usize) -> Option<u8> {
        self.trees.get(self.index(x, y)?).copied()
    }

    fn rows(&self) -> LinesIter {
        LinesIter {
            grid: self,
            pos: (0, 0),
            pos_back: (self.width - 1, 0),
            dir: (0, 1),
            line_dir: (1, 0),
        }
    }

    fn columns(&self) -> LinesIter {
        LinesIter {
            grid: self,
            pos: (0, 0),
            pos_back: (0, self.height - 1),
            dir: (1, 0),
            line_dir: (0, 1),
        }
    }

    fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(x + y * self.width)
    }

    fn visibility(&self) -> VisibilityMap {
        let mut visible = vec![false; self.trees.len()];

        for line in self.rows() {
            self.mark_visible(&mut visible, line.clone());
            self.mark_visible(&mut visible, line.rev());
        }

        for line in self.columns() {
            self.mark_visible(&mut visible, line.clone());
            self.mark_visible(&mut visible, line.rev());
        }

        VisibilityMap {
            visible,
            width: self.width,
            _height: self.height,
        }
    }

    fn max_visibility_score(&self) -> usize {
        self.rows()
            .flatten()
            .map(|(_, pos)| self.visibility_score(pos))
            .max()
            .unwrap()
    }

    fn visibility_score(&self, pos: (usize, usize)) -> usize {
        self.view_distance(pos, LookDir::Up)
            * self.view_distance(pos, LookDir::Right)
            * self.view_distance(pos, LookDir::Down)
            * self.view_distance(pos, LookDir::Left)
    }

    fn view_distance(&self, pos: (usize, usize), dir: LookDir) -> usize {
        let mut view_dir = self.view_dir(pos, dir);

        let (start_tree, _) = view_dir.next().unwrap();

        let mut view_distance = 0;

        for (tree, _) in view_dir {
            view_distance += 1;
            if tree >= start_tree {
                break;
            }
        }

        view_distance
    }

    fn view_dir(&self, pos: (usize, usize), dir: LookDir) -> LineIter {
        match dir {
            LookDir::Up => LineIter {
                grid: self,
                pos,
                pos_back: (pos.0, 0),
                dir: (0, -1),
                is_finished: false,
            },
            LookDir::Right => LineIter {
                grid: self,
                pos,
                pos_back: (self.width - 1, pos.1),
                dir: (1, 0),
                is_finished: false,
            },
            LookDir::Down => LineIter {
                grid: self,
                pos,
                pos_back: (pos.0, self.height - 1),
                dir: (0, 1),
                is_finished: false,
            },
            LookDir::Left => LineIter {
                grid: self,
                pos,
                pos_back: (0, pos.1),
                dir: (-1, 0),
                is_finished: false,
            },
        }
    }

    fn mark_visible(&self, visible: &mut [bool], line: impl Iterator<Item = (u8, (usize, usize))>) {
        let mut height = -1;
        for (tree, pos) in line {
            let tree = tree as i32;
            if height < tree {
                height = tree;
                visible[self.index(pos.0, pos.1).unwrap()] = true;
            }
        }
    }
}

impl FromStr for TreeGrid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut trees = vec![];
        let mut width = 0;
        let mut height = 0;
        for line in s.lines() {
            width = line.len();
            for tree in line.chars() {
                let height = tree.to_digit(10).unwrap();
                trees.push(height as u8);
            }
            height += 1;
        }

        let grid = Self {
            trees,
            height,
            width,
        };
        Ok(grid)
    }
}

struct LinesIter<'a> {
    grid: &'a TreeGrid,
    pos: (usize, usize),
    pos_back: (usize, usize),
    dir: (usize, usize),
    line_dir: (isize, isize),
}

impl<'a> Iterator for LinesIter<'a> {
    type Item = LineIter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.grid.tree(self.pos.0, self.pos.1)?;

        let line = LineIter {
            grid: self.grid,
            pos: self.pos,
            pos_back: self.pos_back,
            dir: self.line_dir,
            is_finished: false,
        };
        self.pos.0 += self.dir.0;
        self.pos.1 += self.dir.1;
        self.pos_back.0 += self.dir.0;
        self.pos_back.1 += self.dir.1;
        Some(line)
    }
}

#[derive(Clone)]
struct LineIter<'a> {
    grid: &'a TreeGrid,
    pos: (usize, usize),
    pos_back: (usize, usize),
    dir: (isize, isize),
    is_finished: bool,
}

impl<'a> Iterator for LineIter<'a> {
    type Item = (u8, (usize, usize));

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_finished {
            return None;
        }

        if self.pos == self.pos_back {
            self.is_finished = true;
        }

        let line = self.grid.tree(self.pos.0, self.pos.1)?;
        let prev_pos = self.pos;

        self.pos.0 = (self.pos.0 as isize + self.dir.0) as usize;
        self.pos.1 = (self.pos.1 as isize + self.dir.1) as usize;

        Some((line, prev_pos))
    }
}

impl<'a> DoubleEndedIterator for LineIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.is_finished {
            return None;
        }

        if self.pos == self.pos_back {
            self.is_finished = true;
        }

        let line = self.grid.tree(self.pos_back.0, self.pos_back.1)?;
        let prev_pos = self.pos_back;

        let x_is_at_end = self.pos_back.0 == 0 && self.dir.0 > 0;
        let y_is_at_end = self.pos_back.1 == 0 && self.dir.1 > 0;
        if !x_is_at_end && !y_is_at_end {
            self.pos_back.0 = (self.pos_back.0 as isize - self.dir.0) as usize;
            self.pos_back.1 = (self.pos_back.1 as isize - self.dir.1) as usize;
        } else {
            self.is_finished = true;
        }

        Some((line, prev_pos))
    }
}

pub fn day8(content: String) {
    println!();
    println!("==== Day 8 ====");
    let grid = content.parse::<TreeGrid>().unwrap();
    let visibility = grid.visibility();

    println!("Part 1");
    let visible = visibility.count_visible();
    println!("Visible: {}", visible);

    println!("Part 2");
    println!("Best view score: {}", grid.max_visibility_score());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r#"30373
25512
65332
33549
35390"#;

    const SIMPLE: &'static str = r#"123
405
678"#;

    #[test]
    fn test_part_1_simple() {
        let grid = SIMPLE.parse::<TreeGrid>().unwrap();
        let visibility = grid.visibility();
        assert_eq!(visibility.count_visible(), 8);
    }

    #[test]
    fn test_part_1() {
        let grid = EXAMPLE.parse::<TreeGrid>().unwrap();
        let visibility = grid.visibility();
        assert_eq!(visibility.count_visible(), 21);
    }

    #[test]
    fn test_part_2() {
        let grid = EXAMPLE.parse::<TreeGrid>().unwrap();
        let max_visibility = grid.max_visibility_score();
        assert_eq!(max_visibility, 8);
    }
}
