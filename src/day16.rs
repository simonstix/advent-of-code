use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete;
use nom::multi::separated_list1;
use nom::{Finish, IResult};
use pathfinding::prelude::dijkstra;
use std::collections::{BTreeSet, HashMap};
use std::str::FromStr;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
struct DuoSearchNode {
    me: usize,
    elephant: usize,
    time: u64,
    pressure: PressureTracker,
}

impl DuoSearchNode {
    fn new(node: usize) -> Self {
        Self {
            me: node,
            elephant: node,
            ..Default::default()
        }
    }

    fn successors(&self, graph: &Graph) -> Vec<(Self, u64)> {
        let time = self.time + 1;

        let current_cost = self.cost(graph);

        // We're done!
        if self.cost(graph) == 0 {
            return vec![(
                Self {
                    me: self.me,
                    elephant: self.elephant,
                    time,
                    pressure: self.pressure.clone(),
                },
                current_cost,
            )];
        }

        // Both move
        let my_moves = graph.neighbors(self.me);
        let elephant_moves = graph.neighbors(self.elephant);
        let mut successors = permutations(my_moves, elephant_moves)
            .map(|(&me, &elephant)| Self {
                me,
                elephant,
                time,
                pressure: self.pressure.clone(),
            })
            .map(|x| (x, current_cost))
            .collect_vec();

        // I open valve
        for &elephant in elephant_moves {
            let mut pressure = self.pressure.clone();
            if pressure.open_valve(self.me, graph) {
                successors.push((
                    Self {
                        me: self.me,
                        elephant,
                        time,
                        pressure,
                    },
                    current_cost,
                ));
            }
        }

        // Elephant opens valve
        for &me in my_moves {
            let mut pressure = self.pressure.clone();
            if pressure.open_valve(self.elephant, graph) {
                successors.push((
                    Self {
                        me,
                        elephant: self.elephant,
                        time,
                        pressure,
                    },
                    current_cost,
                ));
            }
        }

        // Both open valve
        let mut pressure = self.pressure.clone();
        if self.me != self.elephant
            && pressure.can_open_valve(self.me, graph)
            && pressure.can_open_valve(self.elephant, graph)
        {
            assert!(pressure.open_valve(self.me, graph));
            assert!(pressure.open_valve(self.elephant, graph));
            successors.push((
                Self {
                    me: self.me,
                    elephant: self.elephant,
                    time,
                    pressure,
                },
                current_cost,
            ));
        }

        // println!("Current {:?}, next: {:?}", self, successors);
        successors
    }

    fn cost(&self, graph: &Graph) -> u64 {
        graph.all_valves_open - self.score(graph)
    }

    fn score(&self, graph: &Graph) -> u64 {
        self.pressure.pressure_released(graph)
    }
}

fn permutations<'a, A, B>(first: &'a [A], second: &'a [B]) -> impl Iterator<Item = (&'a A, &'a B)> {
    first
        .iter()
        .flat_map(|a| second.iter().map(move |b| (a, b)))
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
struct SearchNode {
    node: usize,
    time: u64,
    pressure: PressureTracker,
}

impl SearchNode {
    fn new(node: usize) -> Self {
        Self {
            node,
            ..Default::default()
        }
    }

    fn successors(&self, graph: &Graph) -> Vec<(Self, u64)> {
        let time = self.time + 1;

        let current_cost = self.cost(graph);

        // We're done!
        if current_cost == 0 {
            return vec![(
                Self {
                    node: self.node,
                    time,
                    pressure: self.pressure.clone(),
                },
                current_cost,
            )];
        }

        let mut successors = graph
            .neighbors(self.node)
            .iter()
            .map(|x| Self {
                node: *x,
                time,
                pressure: self.pressure.clone(),
            })
            .map(|x| (x, current_cost))
            .collect_vec();

        // Open valve
        let mut pressure = self.pressure.clone();
        if pressure.open_valve(self.node, graph) {
            successors.push((
                Self {
                    node: self.node,
                    time,
                    pressure,
                },
                current_cost,
            ));
        }

        // println!("Current {:?}, next: {:?}", self, successors);
        successors
    }

    fn cost(&self, graph: &Graph) -> u64 {
        graph.all_valves_open - self.score(graph)
    }

    fn score(&self, graph: &Graph) -> u64 {
        self.pressure.pressure_released(graph)
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
struct PressureTracker {
    open_valves: BTreeSet<usize>,
}

impl PressureTracker {
    pub fn can_open_valve(&self, node: usize, graph: &Graph) -> bool {
        graph.flow_rate(node) > 0 && !self.open_valves.contains(&node)
    }

    pub fn open_valve(&mut self, node: usize, graph: &Graph) -> bool {
        graph.flow_rate(node) > 0 && self.open_valves.insert(node)
    }
    pub fn pressure_released(&self, graph: &Graph) -> u64 {
        self.open_valves.iter().map(|x| graph.flow_rate(*x)).sum()
    }
}

#[derive(Debug, Default)]
struct Graph {
    start: usize,
    nodes: Vec<Valve>,
    edges: HashMap<usize, Vec<usize>>,
    all_valves_open: u64,
}

impl Graph {
    fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, valve: Valve) -> usize {
        let id = self.nodes.len();
        self.nodes.push(valve);
        id
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.entry(from).or_default().push(to);
    }

    pub fn flow_rate(&self, node: usize) -> u64 {
        self.nodes.get(node).unwrap().flow_rate
    }

    pub fn neighbors(&self, from: usize) -> &[usize] {
        self.edges.get(&from).unwrap()
    }

    pub fn optimal_pressure_release(&self, max_time: u64) -> u64 {
        let (path, cost) = dijkstra(
            &SearchNode::new(self.start),
            |x| x.successors(self),
            |x| x.time >= max_time,
        )
        .expect("No goal found");

        // print_path(&path, self);

        // Revert cost to get released pressure
        let score = (max_time * self.all_valves_open) - cost;
        let recalc_score = path.iter().rev().skip(1).map(|x| x.score(self)).sum();
        assert_eq!(score, recalc_score);
        score
    }

    #[allow(dead_code)]
    pub fn duo_optimal_pressure_release(&self, max_time: u64) -> u64 {
        let (path, cost) = dijkstra(
            &DuoSearchNode::new(self.start),
            |x| x.successors(self),
            |x| x.time >= max_time,
        )
        .expect("No goal found");

        // print_duo_path(&path, self);

        // Revert cost to get released pressure
        let score = (max_time * self.all_valves_open) - cost;
        let recalc_score = path.iter().rev().skip(1).map(|x| x.score(self)).sum();
        assert_eq!(score, recalc_score);
        score
    }
}

#[allow(dead_code)]
fn print_path(path: &[SearchNode], graph: &Graph) {
    for window in path.windows(2) {
        let (from, to): (&SearchNode, &SearchNode) = window.iter().collect_tuple().unwrap();

        println!("== Minute {} ==", from.time + 1);
        if from.pressure.open_valves.is_empty() {
            println!("No valves are open.");
        } else {
            println!(
                "Valve ?? is open, releasing {} pressure.",
                from.pressure.pressure_released(graph)
            );
        }

        let from_name = &graph.nodes.get(from.node).unwrap().name;
        let to_name = &graph.nodes.get(to.node).unwrap().name;
        if from.node == to.node {
            println!("You open valve {}", from_name);
        } else {
            println!("You move to valve {}", to_name);
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_duo_path(path: &[DuoSearchNode], graph: &Graph) {
    for window in path.windows(2) {
        let (from, to): (&DuoSearchNode, &DuoSearchNode) = window.iter().collect_tuple().unwrap();

        println!("== Minute {} ==", from.time + 1);
        if from.pressure.open_valves.is_empty() {
            println!("No valves are open.");
        } else {
            println!(
                "Valve ?? is open, releasing {} pressure.",
                from.pressure.pressure_released(graph)
            );
        }

        let from_name = &graph.nodes.get(from.me).unwrap().name;
        let to_name = &graph.nodes.get(to.me).unwrap().name;
        if from.me == to.me {
            println!("You open valve {}", from_name);
        } else {
            println!("You move to valve {}", to_name);
        }
        let from_name = &graph.nodes.get(from.elephant).unwrap().name;
        let to_name = &graph.nodes.get(to.elephant).unwrap().name;
        if from.elephant == to.elephant {
            println!("Elephant open valve {}", from_name);
        } else {
            println!("Elephant move to valve {}", to_name);
        }
        println!();
    }
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: u64,
}

impl Valve {
    pub fn new(name: String, flow_rate: u64) -> Self {
        Self { name, flow_rate }
    }
}

fn parse_graph(graph: &str) -> Graph {
    let definitions = graph
        .lines()
        .map(|x| x.parse::<ValveDefinition>().unwrap())
        .collect_vec();

    let mut graph = Graph::new();

    let name_to_id: HashMap<_, _> = definitions
        .iter()
        .map(|x| {
            (
                x.name.clone(),
                graph.add_node(Valve::new(x.name.clone(), x.flow_rate)),
            )
        })
        .collect();

    for definition in definitions {
        let from_id = *name_to_id.get(&definition.name).unwrap();
        for neighbor in &definition.neighbors {
            let to_id = *name_to_id.get(neighbor).unwrap();

            graph.add_edge(from_id, to_id);
        }

        graph.all_valves_open += definition.flow_rate;
    }

    graph.start = *name_to_id.get("AA").unwrap();

    graph
}

struct ValveDefinition {
    name: String,
    flow_rate: u64,
    neighbors: Vec<String>,
}

fn parse_valve_definition(input: &str) -> IResult<&str, ValveDefinition> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, name) = take(2usize)(input)?;
    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, flow_rate) = complete::u64(input)?;
    let (input, _) = tag("; ")(input)?;
    let (input, _) = alt((
        tag("tunnel leads to valve "),
        tag("tunnels lead to valves "),
    ))(input)?;
    let (input, neighbors) = separated_list1(tag(", "), take(2usize))(input)?;
    Ok((
        input,
        ValveDefinition {
            name: name.to_owned(),
            flow_rate,
            neighbors: neighbors.into_iter().map(|x| x.to_owned()).collect(),
        },
    ))
}

impl FromStr for ValveDefinition {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_valve_definition(s).finish() {
            Ok((_remaining, definitions)) => Ok(definitions),
            Err(nom::error::Error { input, code }) => Err(nom::error::Error {
                input: input.to_owned(),
                code,
            }),
        }
    }
}

pub fn day16(content: String) {
    println!();
    println!("==== Day 16 ====");
    let graph = parse_graph(&content);

    println!("Part 1");
    println!(
        "Optimal pressure release: {}",
        graph.optimal_pressure_release(30)
    );

    println!();
    println!("Part 2");
    println!("Skipping Part 2");
    // println!(
    //     "Optimal duo pressure release: {}",
    //     graph.duo_optimal_pressure_release(26)
    // );
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;

    #[test]
    fn test_part_1() {
        let graph = parse_graph(EXAMPLE);
        assert_eq!(graph.optimal_pressure_release(30), 1651);
    }

    #[test]
    fn test_part_2() {
        let graph = parse_graph(EXAMPLE);
        println!("graph {:#?}", graph);
        assert_eq!(graph.duo_optimal_pressure_release(26), 1707);
    }
}
