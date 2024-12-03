use itertools::Itertools;
use sliding_windows::{IterExt, Storage};
use std::{path::Iter, time::Instant, vec::Vec};

const TEST_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

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
    let input = super::get_input("../input/input_day2.txt");

    let result = input
        .lines()
        .map(parse_line)
        .map(get_increments)
        .map(is_safe_report)
        .filter(|x| *x)
        .count();
    println!("result part I: {}", result);
}

fn part2() {
    let input = super::get_input("../input/input_day2.txt");

    let result = input
        .lines()
        //.take(10)
        //.inspect(|line| println!("line: \"{line}\""))
        .map(parse_line)
        //.inspect(|x| println!("parsed to: {x:?}"))
        .map(generate_possible_reports)
        .filter_map(any_safe_report)
        //.inspect(|is_safe| println!("is safe: {}\n_________", is_safe))
        //.filter(|x| *x)
        .count();
    println!("result part II: {}", result);
}

type Number = i64;

fn parse_line(line: &str) -> Vec<Number> {
    line.split_whitespace().map(parse_line_values).collect()
}

fn parse_line_values(x: &str) -> Number {
    x.parse::<Number>().unwrap()
}

fn generate_possible_reports(
    report: Vec<Number>,
) -> std::iter::Chain<
    itertools::Combinations<std::vec::IntoIter<i64>>,
    std::array::IntoIter<std::vec::Vec<i64>, 1>,
> {
    let comb_len = report.len() - 1;
    let original = report.clone();
    report.into_iter().combinations(comb_len).chain([original])
}

fn any_safe_report<T: Iterator<Item = Vec<Number>>>(possible_reports: T) -> Option<()> {
    match possible_reports
        .map(get_increments)
        //.inspect(|x| println!(" increments: {:?}", x))
        .any(is_safe_report)
    {
        true => Some(()),
        false => None,
    }
}

fn get_increments(report: Vec<Number>) -> Vec<Number> {
    report
        .windows(2)
        .map(|window| {
            let mut window = window.iter();
            let (a, b) = window.next_tuple().unwrap();
            b - a
        })
        .collect()
}

fn is_safe_report(report: Vec<Number>) -> bool {
    let iter = report.iter();
    let max_abs = iter.clone().max_by(|x, y| x.abs().cmp(&y.abs())).unwrap();
    //println!("max: {}",max_abs);
    let is_growing = iter.clone().all(|x| x.signum().is_positive());
    let is_decreasing = iter.into_iter().all(|x| x.signum().is_negative());

    matches!(max_abs.abs(), 1..=3 if is_growing || is_decreasing)
}
