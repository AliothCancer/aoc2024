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
        .map(parse_line)
        .map(generate_possible_reports)
        .map(any_safe_report)
        .filter(|x| *x)
        .count();
    println!("result part II: {}", result);
}

type Number = i64;

fn parse_line(line: &str) -> Vec<Number> {
    line.split_whitespace()
        .map(|x| x.parse::<Number>().unwrap())
        .collect()
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

fn any_safe_report<T: Iterator<Item = Vec<Number>>>(possible_reports: T) -> bool {
    possible_reports.map(get_increments).any(is_safe_report)
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
    let is_growing = report.iter().all(|x| x.signum().is_positive());
    let is_decreasing = report.iter().all(|x| x.signum().is_negative());
    
    if is_growing{
        (1..=3).contains(report.iter().max().unwrap())
    } else if is_decreasing{
        (-3..=-1).contains(report.iter().min().unwrap())
    } else { false }
}
