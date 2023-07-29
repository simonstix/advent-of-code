use anyhow::Context;
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{char, u64};
use nom::multi::separated_list0;
use nom::{Finish, IResult};
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug)]
struct SignalPair {
    left: Signal,
    right: Signal,
}

impl SignalPair {
    fn check_order(&self) -> bool {
        self.left < self.right
    }
}

impl FromStr for SignalPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let signals = s
            .lines()
            .map(|x| x.parse::<Signal>().context("could not parse signal"))
            .collect::<anyhow::Result<Vec<Signal>>>()?;
        let (left, right) = signals
            .into_iter()
            .collect_tuple()
            .expect("unexpected length");
        Ok(SignalPair { left, right })
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Signal {
    Num(u64),
    List(Vec<Signal>),
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Num(left), Self::Num(right)) => left.cmp(right),
            (Self::List(left), Self::List(right)) => {
                for (left, right) in left.iter().zip(right) {
                    match left.cmp(right) {
                        Ordering::Equal => {
                            // Continue checking
                        }
                        order => return order,
                    }
                }

                // Still undecided
                left.len().cmp(&right.len())
            }
            (left @ Self::List(_), Self::Num(right)) => {
                left.cmp(&Signal::List(vec![Self::Num(*right)]))
            }
            (Self::Num(left), right @ Self::List(_)) => {
                Signal::List(vec![Self::Num(*left)]).cmp(right)
            }
        }
    }
}

fn num_signal(input: &str) -> IResult<&str, Signal> {
    let (input, value) = u64(input)?;
    Ok((input, Signal::Num(value)))
}

fn list_signal(input: &str) -> IResult<&str, Signal> {
    let (input, _) = char('[')(input)?;
    let (input, values) = separated_list0(char(','), parse_signal)(input)?;
    let values = Signal::List(values);
    let (input, _) = char(']')(input)?;
    Ok((input, values))
}

fn parse_signal(input: &str) -> IResult<&str, Signal> {
    alt((num_signal, list_signal))(input)
}

impl FromStr for Signal {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match list_signal(s).finish() {
            Ok((_remaining, signal)) => Ok(signal),
            Err(nom::error::Error { input, code }) => Err(nom::error::Error {
                input: input.to_owned(),
                code,
            }),
        }
    }
}

fn signal_order_value(pairs: &[SignalPair]) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter(|(_, pair)| pair.check_order())
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn day13(content: String) {
    println!();
    println!("==== Day 13 ====");

    println!("Part 1");
    let signal_pairs = content
        .split("\n\n")
        .map(|x| x.parse::<SignalPair>().unwrap())
        .collect_vec();
    println!("Signal order value: {}", signal_order_value(&signal_pairs));

    println!();
    println!("Part 2");
    let signals = content
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<Signal>().unwrap())
        .chain(create_divider_packets())
        .sorted()
        .collect_vec();
    println!("Signal decoder key: {}", find_decoder_key(&signals));
}

fn create_divider_packets() -> [Signal; 2] {
    [
        Signal::List(vec![Signal::Num(2)]),
        Signal::List(vec![Signal::Num(6)]),
    ]
}

fn find_decoder_key(list: &[Signal]) -> usize {
    let dividers = create_divider_packets();
    list.iter()
        .enumerate()
        .filter(|(_, x)| dividers.contains(x))
        .map(|(i, _)| i + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    #[test]
    fn test_part_1() {
        let signal_pairs = EXAMPLE
            .split("\n\n")
            .map(|x| x.parse::<SignalPair>().unwrap())
            .collect_vec();
        assert_eq!(signal_order_value(&signal_pairs), 13);
    }

    #[test]
    fn test_part_2() {
        let signals = EXAMPLE
            .lines()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<Signal>().unwrap())
            .chain(create_divider_packets())
            .sorted()
            .collect_vec();
        assert_eq!(find_decoder_key(&signals), 140);
    }
}
