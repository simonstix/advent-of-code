use crate::utils::manhattan_distance;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::{Finish, IResult};
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::FromStr;

type Point2 = na::Point2<i64>;

#[derive(Debug)]
struct Sensor {
    pos: Point2,
    closest_beacon: Point2,
    #[allow(dead_code)]
    distance_to_closest_beacon: usize,
}

impl Sensor {
    fn new(pos: Point2, closest_beacon: Point2) -> Self {
        Self {
            distance_to_closest_beacon: manhattan_distance(&pos, &closest_beacon) as usize,
            pos,
            closest_beacon,
        }
    }

    fn distance_to_closest_beacon(&self) -> usize {
        self.distance_to_closest_beacon
    }
}

#[allow(dead_code)]
fn count_row_positions_without_beacon(
    sensors: &[Sensor],
    x_coords: impl Iterator<Item = i64>,
    y: i64,
) -> usize {
    let sensor_positions: HashSet<_> = sensors.iter().map(|x| x.pos).collect();
    let beacon_positions: HashSet<_> = sensors.iter().map(|x| x.closest_beacon).collect();
    let mut count = 0;
    for x in x_coords {
        let pos = Point2::new(x, y);

        if sensor_positions.contains(&pos) || beacon_positions.contains(&pos) {
            continue;
        }

        if sensors
            .iter()
            .any(|x| x.distance_to_closest_beacon() >= manhattan_distance(&pos, &x.pos) as usize)
        {
            count += 1;
        }
    }
    count
}

fn first_empty_spot(
    sensors: &[Sensor],
    x_range: RangeInclusive<i64>,
    y_range: RangeInclusive<i64>,
) -> Option<Point2> {
    let mut y = 0;
    while y <= *y_range.end() {
        let mut x = 0;
        'row: while x <= *x_range.end() {
            let pos = Point2::new(x, y);

            for sensor in sensors {
                let closest_sensor = sensor.distance_to_closest_beacon();
                let current_distance = manhattan_distance(&pos, &sensor.pos) as usize;

                if closest_sensor >= current_distance {
                    if sensor.pos.x > x {
                        // Mirror around sensor
                        x += (sensor.pos.x - x) + 1;
                    } else {
                        // Move to end of manhattan distance
                        x += (closest_sensor - current_distance + 1) as i64;
                    }
                    continue 'row;
                }
            }

            // No sensor in range
            return Some(pos);
        }
        y += 1;
    }

    None
}

fn parse_sensor(input: &str) -> IResult<&str, Sensor> {
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, sensor_x) = complete::i64(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, sensor_y) = complete::i64(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, beacon_x) = complete::i64(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, beacon_y) = complete::i64(input)?;

    Ok((
        input,
        Sensor::new(
            Point2::new(sensor_x, sensor_y),
            Point2::new(beacon_x, beacon_y),
        ),
    ))
}

impl FromStr for Sensor {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_sensor(s).finish() {
            Ok((_remaining, sensor)) => Ok(sensor),
            Err(nom::error::Error { input, code }) => Err(nom::error::Error {
                input: input.to_owned(),
                code,
            }),
        }
    }
}

pub fn day15(content: String) {
    println!();
    println!("==== Day 15 ====");
    let sensors = content
        .lines()
        .map(|x| x.parse::<Sensor>().unwrap())
        .collect_vec();

    println!("Part 1");

    println!("Skipped part 1 for performance");

    // let start = Instant::now();
    // for r in [10000000] {
    //     let range = -r..r;
    //     println!(
    //         "Positions without beacon in range {:?}: {}",
    //         range.clone(),
    //         count_row_positions_without_beacon(&sensors, range, 2000000),
    //     );
    // }
    // println!(
    //     "Duration: {}",
    //     Instant::now().duration_since(start).as_secs_f64()
    // );

    println!();
    println!("Part 2");
    let first_empty_spot = first_empty_spot(&sensors, 0..=4000000, 0..=4000000).unwrap();
    println!(
        "Missing beacon: {}",
        calc_tuning_frequency(first_empty_spot)
    );
}

fn calc_tuning_frequency(pos: Point2) -> i64 {
    pos.x * 4000000 + pos.y
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    const EXAMPLE: &'static str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;

    #[test]
    fn test_part_1() {
        let sensors = EXAMPLE
            .lines()
            .map(|x| x.parse::<Sensor>().unwrap())
            .collect_vec();

        assert_eq!(
            count_row_positions_without_beacon(&sensors, -10..30, 10),
            26
        );
    }

    #[test]
    fn test_part_2() {
        let sensors = EXAMPLE
            .lines()
            .map(|x| x.parse::<Sensor>().unwrap())
            .collect_vec();

        let pos = first_empty_spot(&sensors, 0..=20, 0..=20);
        assert_eq!(pos, Some(Point2::new(14, 11)));
        assert_eq!(calc_tuning_frequency(pos.unwrap()), 56000011);
    }
}
