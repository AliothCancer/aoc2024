#![allow(unused)]

use itertools::Itertools;
use sliding_windows::{IterExt, Storage};
use std::{path::Iter, time::Instant, vec::Vec};

pub fn run() {
    //let i = Instant::now();
    //part1();
    //let duration = i.elapsed();
    //println!("{} seconds", duration.as_nanos() as f64 / 1_000_000_000.0);

    //let i = Instant::now();
    part2();
    //let duration = i.elapsed();
    //println!("{} seconds", duration.as_nanos() as f64 /1_000_000_000.0);
}

fn part1() {
    let input = super::get_input("../input/input_day2.txt");
    let test = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    let p1_result = input
        .lines()
        //.skip(4)
        //.take(2)
        //.enumerate()
        .map(|line| {
            let mut storage = Storage::new(2);
            line.split(" ")
                .map(|value| value.parse::<i64>().unwrap())
                .sliding_windows(&mut storage)
                .map(|x| {
                    let mut x = x.into_iter();
                    let x1 = x.next().unwrap();
                    let x2 = x.next().unwrap();
                    let delta = x2 - x1;
                    let signum = delta.signum();
                    //println!("{x2}-{x1} = {delta} (segno:{signum})");
                    match delta.abs() {
                        1..=3 => (signum, delta),
                        _ => (-signum, 0),
                    }
                })
                .collect::<Vec<_>>()
        })
        .filter(|x| x.iter().all(|y| y.0 == x[0].0))
        .collect::<Vec<_>>();

    for i in p1_result.iter() {
        println!(" {:?}", i);
    }
    println!("{}", p1_result.len());
}

fn _part1_test() {
    let test = "\
7 6 4 2 1
1 2 3 4 5
1 2 7 8 9
9 7 6 2 1
1 1 1 1 1
1 3 6 7 9";

    let p1_result = test
        .lines()
        .map(|line| {
            let mut storage = Storage::new(2);
            line.split(" ")
                .map(|value| {
                    let string = "Il valore parsato male: ".to_owned() + value;
                    value.parse::<i64>().expect(&string)
                })
                .sliding_windows(&mut storage)
                .map(|x| {
                    let mut x = x.into_iter();
                    let (x1, x2) = (x.next().unwrap(), x.next().unwrap());
                    x1 - x2
                })
                .collect::<Vec<_>>()
        })
        .enumerate()
        .filter(|(_, report)| {
            report.iter().map(|x| x.abs()).max().unwrap() <= 3
                && report.iter().product::<i64>().is_positive()
        })
        .collect::<Vec<_>>();
    println!("{:?}", p1_result);
}

fn part2() {
    let _input = super::get_input("../input/input_day2.txt");
    let test = "\
7 6 4 2 1
1 2 3 4 5
1 2 7 8 9
9 7 6 2 1
1 1 1 1 1
1 3 6 7 9";
    let result = test
        .lines()
        //.take(1)
        .inspect(|line| println!("line: \"{line}\""))
        .map(parse_line)
        //.inspect(|x| println!("parsed to: {x:?}"))
        .map(generate_possible_reports)
        .map(any_safe_report)
        .inspect(|is_safe| println!("is safe: {}", is_safe))
        .filter(|x| *x)
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

fn generate_possible_reports(report: Vec<Number>) -> std::iter::Chain<itertools::Combinations<std::vec::IntoIter<i64>>, std::array::IntoIter<std::vec::Vec<i64>, 1>> {
    let comb_len = report.len() - 1;
    let original = report.clone();
    report.into_iter().combinations(comb_len).chain([original])
}

fn any_safe_report<T: Iterator<Item = Vec<Number>>>(possible_reports: T) -> bool {
    possible_reports
        .map(get_increments)
        .inspect(|x| println!(" increments: {:?}", x))
        .any(is_safe_report)
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
    let is_max_abs3 = iter.clone().map(|x| x.abs()).all(|x| x > 1 && x <= 3);
    let is_monotone = iter.map(|x|x.signum()).collect::<Vec<_>>();
    println!("segni: {:?}",is_monotone);
    is_max_abs3 && is_monotone.iter().unique().count() == 1
}
