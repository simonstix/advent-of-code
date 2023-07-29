use anyhow::bail;
use itertools::Itertools;
use std::str::FromStr;

enum Outcome {
    Loss,
    Draw,
    Win,
}

impl FromStr for Outcome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let outcome = match s {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => bail!("unsupported str"),
        };
        Ok(outcome)
    }
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }

    fn achieve_outcome(&self, against: &Hand) -> Hand {
        match (self, against) {
            (Outcome::Loss, Hand::Rock) => Hand::Scissor,
            (Outcome::Loss, Hand::Paper) => Hand::Rock,
            (Outcome::Loss, Hand::Scissor) => Hand::Paper,
            (Outcome::Draw, hand) => hand.clone(),
            (Outcome::Win, Hand::Rock) => Hand::Paper,
            (Outcome::Win, Hand::Paper) => Hand::Scissor,
            (Outcome::Win, Hand::Scissor) => Hand::Rock,
        }
    }
}

#[derive(Clone)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}

impl Hand {
    fn score(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissor => 3,
        }
    }

    fn result(&self, other: &Hand) -> Outcome {
        match (self, other) {
            (Hand::Rock, Hand::Rock) => Outcome::Draw,
            (Hand::Rock, Hand::Paper) => Outcome::Loss,
            (Hand::Rock, Hand::Scissor) => Outcome::Win,
            (Hand::Paper, Hand::Rock) => Outcome::Win,
            (Hand::Paper, Hand::Paper) => Outcome::Draw,
            (Hand::Paper, Hand::Scissor) => Outcome::Loss,
            (Hand::Scissor, Hand::Rock) => Outcome::Loss,
            (Hand::Scissor, Hand::Paper) => Outcome::Win,
            (Hand::Scissor, Hand::Scissor) => Outcome::Draw,
        }
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hand = match s {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissor,
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissor,
            _ => bail!("unsupported str"),
        };
        Ok(hand)
    }
}

pub fn day2(content: String) -> anyhow::Result<()> {
    println!("Day 1");
    let hands = content
        .split('\n')
        .map(|x| {
            let values = x.split(' ').collect_vec();
            assert_eq!(values.len(), 2);
            let enemy = values[0].parse::<Hand>().unwrap();
            let your_outcome = values[1].parse::<Outcome>().unwrap();
            let you = your_outcome.achieve_outcome(&enemy);

            (enemy, you)
        })
        .collect_vec();

    let your_score: usize = hands
        .iter()
        .map(|(enemy, you)| calc_score(you, enemy))
        .sum();
    println!("Your score: {}", your_score);
    println!();

    Ok(())
}

fn calc_score(you: &Hand, enemy: &Hand) -> usize {
    you.score() + you.result(enemy).score()
}
