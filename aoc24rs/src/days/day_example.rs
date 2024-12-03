#![allow(unused, clippy::diverging_sub_expression, clippy::let_unit_value)]

use std::time::Instant;

const INPUT_PATH: &str = "../input/input_day.txt";
const TEST_INPUT: &str = "";

pub fn run() {
    let i = Instant::now();
    part1();
    let duration = i.elapsed();
    println!("{} seconds", duration.as_nanos() as f64 / 1_000_000_000.0);

    let i = Instant::now();
    part2();
    let duration = i.elapsed();
    println!("{} seconds", duration.as_nanos() as f64 / 1_000_000_000.0);
}

fn part1() {
    let input = super::get_input(INPUT_PATH);

    let result = unimplemented!();
    println!("result part I: {:?}", result);
}

fn part2() {
    let input = super::get_input(INPUT_PATH);

    let result = unimplemented!();
    println!("result part II: {:?}", result);
}
