use itertools::Itertools;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug)]
struct Pair {
    first: RangeInclusive<usize>,
    second: RangeInclusive<usize>,
}

impl Pair {
    fn contains_other(&self) -> bool {
        range_contains_other(&self.first, &self.second)
            || range_contains_other(&self.second, &self.first)
    }

    fn has_overlap(&self) -> bool {
        self.first.contains(self.second.start())
            || self.first.contains(self.second.end())
            || self.second.contains(self.first.start())
            || self.second.contains(self.first.end())
    }
}

fn range_contains_other(first: &RangeInclusive<usize>, second: &RangeInclusive<usize>) -> bool {
    first.start() <= second.start() && second.end() <= first.end()
}

impl FromStr for Pair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s
            .split(',')
            .map(|x| {
                let (first, second) = x
                    .split('-')
                    .map(|n| n.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                first..=second
            })
            .collect_tuple()
            .unwrap();

        Ok(Self { first, second })
    }
}

pub fn day4(content: String) {
    let pairs = content
        .lines()
        .map(|x| x.parse::<Pair>().unwrap())
        .collect_vec();
    println!("Total pairs: {}", pairs.len());

    println!();
    println!("==== Day 4 ====");
    println!("Part 1");
    let contained_pairs = pairs.iter().filter(|x| x.contains_other()).count();
    println!("Contained pairs: {}", contained_pairs);
    println!();
    println!("Part 2");
    let overlapping_pairs: usize = pairs.iter().filter(|x| x.has_overlap()).count();
    println!("Overlapping pairs: {}", overlapping_pairs);
}
