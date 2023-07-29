use crate::utils::dfs;
use derivative::Derivative;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{multispace1, space1, u64};
use nom::combinator::fail;
use nom::multi::separated_list1;
use nom::{Finish, IResult};
use std::ops::AddAssign;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

fn parse_resource(input: &str) -> IResult<&str, Resource> {
    let (input, resource) = alt((tag("ore"), tag("clay"), tag("obsidian"), tag("geode")))(input)?;
    let resource = match resource {
        "ore" => Resource::Ore,
        "clay" => Resource::Clay,
        "obsidian" => Resource::Obsidian,
        "geode" => Resource::Geode,
        _ => return fail(input),
    };

    Ok((input, resource))
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Robot {
    costs: Vec<(Resource, u64)>,
    produces: Resource,
}

fn parse_cost(input: &str) -> IResult<&str, (Resource, u64)> {
    let (input, cost) = u64(input)?;
    let (input, _) = space1(input)?;
    let (input, resource) = parse_resource(input)?;
    Ok((input, (resource, cost)))
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    let (input, _) = tag("Each ")(input)?;
    let (input, produces) = parse_resource(input)?;
    let (input, _) = tag(" robot costs ")(input)?;
    let (input, costs) = separated_list1(tag(" and "), parse_cost)(input)?;
    let (input, _) = tag(".")(input)?;
    Ok((input, Robot { costs, produces }))
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Blueprint {
    id: u64,
    robots: Vec<Robot>,
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, _) = tag("Blueprint ")(input)?;
    let (input, id) = u64(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, robots) = separated_list1(multispace1, parse_robot)(input)?;
    Ok((input, Blueprint { id, robots }))
}

fn parse_blueprints(input: &str) -> Vec<Blueprint> {
    let (input, blueprints) = separated_list1(multispace1, parse_blueprint)(input)
        .finish()
        .unwrap();
    assert_eq!(input, "");
    blueprints
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
struct Storage {
    ore: u64,
    clay: u64,
    obsidian: u64,
    geode: u64,
}

impl Storage {
    fn add_resource(&mut self, resource: &Resource, amount: u64) {
        match resource {
            Resource::Ore => {
                self.ore += amount;
            }
            Resource::Clay => {
                self.clay += amount;
            }
            Resource::Obsidian => {
                self.obsidian += amount;
            }
            Resource::Geode => {
                self.geode += amount;
            }
        }
    }

    fn resource(&self, resource: &Resource) -> u64 {
        match resource {
            Resource::Ore => self.ore,
            Resource::Clay => self.clay,
            Resource::Obsidian => self.obsidian,
            Resource::Geode => self.geode,
        }
    }

    fn remove_resource(&mut self, resource: &Resource, amount: u64) {
        match resource {
            Resource::Ore => {
                self.ore -= amount;
            }
            Resource::Clay => {
                self.clay -= amount;
            }
            Resource::Obsidian => {
                self.obsidian -= amount;
            }
            Resource::Geode => {
                self.geode -= amount;
            }
        }
    }
}

impl AddAssign for Storage {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}

#[derive(Clone, Derivative)]
#[derivative(Debug, Eq, PartialEq, Hash)]
struct Simulator<'a> {
    #[derivative(Debug = "ignore", PartialEq = "ignore", Hash = "ignore")]
    blueprint: &'a Blueprint,
    time: usize,
    storage: Storage,
    production: Storage,
}

impl<'a> Simulator<'a> {
    fn new(blueprint: &'a Blueprint) -> Self {
        Self {
            time: 0,
            blueprint,
            storage: Default::default(),
            production: Storage {
                ore: 1,
                ..Default::default()
            },
        }
    }

    fn successors(&self) -> impl Iterator<Item = Simulator<'a>> + '_ {
        // Don't build anything
        std::iter::once(self.next()).chain(
            // Build robots
            self.blueprint
                .robots
                .iter()
                // Basically a if x return, but with the same return type
                .filter_map(|robot| self.next_with_built_robot(robot)),
        )
    }

    fn next_with_built_robot(&self, robot: &Robot) -> Option<Self> {
        if robot
            .costs
            .iter()
            .all(|(resource, cost)| self.storage.resource(resource) >= *cost)
        {
            let mut next = self.next();
            for (resource, cost) in &robot.costs {
                next.storage.remove_resource(resource, *cost);
            }
            next.production.add_resource(&robot.produces, 1);
            Some(next)
        } else {
            None
        }
    }

    fn next(&self) -> Self {
        let mut clone = self.clone();
        clone.time += 1;
        clone.storage += clone.production.clone();
        clone
    }

    fn score(&self) -> u64 {
        self.storage.geode
    }

    fn best_possible_score(&self, max_time: usize) -> u64 {
        let mut score = self.score();
        let mut production = self.production.geode;
        for _ in self.time..max_time {
            score += production;

            // Add a new geode bot every minute
            production += 1;
        }
        score
    }
}

fn score_blueprints(blueprints: &[Blueprint], max_time: usize, with_quality: bool) -> u64 {
    let mut total_score = 0;
    for blueprint in blueprints {
        let simulator = Simulator::new(blueprint);
        let best_score = dfs(
            simulator,
            |x| x.successors().collect_vec(),
            |x| x.score(),
            |x| x.best_possible_score(max_time),
            |x| x.time >= max_time,
        );
        if with_quality {
            total_score += best_score * blueprint.id;
        } else if total_score == 0 {
            total_score = best_score;
        } else {
            total_score *= best_score;
        }
    }
    total_score
}

pub fn day19(content: String) {
    println!();
    println!("==== Day 19 ====");
    let blueprints = parse_blueprints(&content);

    println!("Part 1");
    println!("Skipping part 1");
    // println!("Score: {}", score_blueprints(&blueprints, 24, true));

    println!();
    println!("Part 2");
    println!("Skipping part 2");
    // println!(
    //     "Score: {}",
    //     score_blueprints(&blueprints.into_iter().take(3).collect_vec(), 32, false)
    // );
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r#"Blueprint 1:
  Each ore robot costs 4 ore.
  Each clay robot costs 2 ore.
  Each obsidian robot costs 3 ore and 14 clay.
  Each geode robot costs 2 ore and 7 obsidian.

Blueprint 2:
  Each ore robot costs 2 ore.
  Each clay robot costs 3 ore.
  Each obsidian robot costs 3 ore and 8 clay.
  Each geode robot costs 3 ore and 12 obsidian."#;

    #[test]
    fn test_part_1() {
        let blueprints = parse_blueprints(EXAMPLE);
        assert_eq!(score_blueprints(&blueprints, 24, true), 33)
    }

    #[test]
    fn test_part_2() {
        let blueprints = parse_blueprints(EXAMPLE).into_iter().take(3).collect_vec();
        assert_eq!(score_blueprints(&blueprints, 32, false), 56 * 62)
    }
}
