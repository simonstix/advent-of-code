use crate::day1::day1;
use crate::day10::day10;
use crate::day11::day11;
use crate::day12::day12;
use crate::day13::day13;
use crate::day14::day14;
use crate::day15::day15;
use crate::day16::day16;
use crate::day17::day17;
use crate::day18::day18;
use crate::day19::day19;
use crate::day2::day2;
use crate::day20::day20;
use crate::day3::day3;
use crate::day4::day4;
use crate::day5::day5;
use crate::day6::day6;
use crate::day7::day7;
use crate::day8::day8;
use crate::day9::day9;
use std::fs;
use std::io::Read;

extern crate core;
extern crate nalgebra as na;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

fn main() {
    day1(load_to_string("inputs/day1.txt")).unwrap();
    day2(load_to_string("inputs/day2.txt")).unwrap();
    day3(load_to_string("inputs/day3.txt")).unwrap();
    day4(load_to_string("inputs/day4.txt"));
    day5(load_to_string("inputs/day5.txt"));
    day6(load_to_string("inputs/day6.txt"));
    day7(load_to_string("inputs/day7.txt"));
    day8(load_to_string("inputs/day8.txt"));
    day9(load_to_string("inputs/day9.txt"));
    day10(load_to_string("inputs/day10.txt"));
    day11(load_to_string("inputs/day11.txt"));
    day12(load_to_string("inputs/day12.txt"));
    day13(load_to_string("inputs/day13.txt"));
    day14(load_to_string("inputs/day14.txt"));
    day15(load_to_string("inputs/day15.txt"));
    day16(load_to_string("inputs/day16.txt"));
    day17(load_to_string("inputs/day17.txt"));
    day18(load_to_string("inputs/day18.txt"));
    day19(load_to_string("inputs/day19.txt"));
    day20(load_to_string("inputs/day20.txt"));
}

fn load_to_string(path: &str) -> String {
    let mut file = fs::File::open(path).expect("could not open file");
    let mut output = String::new();
    file.read_to_string(&mut output)
        .expect("could not read to string");
    output
}
