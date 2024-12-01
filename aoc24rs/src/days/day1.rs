use std::collections::HashMap;

pub fn run() {
    part1();
    part2();
}

fn part1() {
    let input = super::get_input("../input/input_day1.txt");

    let mut col1 = vec![];
    let mut col2 = vec![];

    input.lines().for_each(|line| {
        let mut k = line.split_ascii_whitespace();
        let c1 = k.next().unwrap().parse::<u64>().unwrap();
        let c2 = k.next().unwrap().parse::<u64>().unwrap();
        col1.push(c1);
        col2.push(c2);
    });

    col1.sort();
    col2.sort();

    let p1_result = col1
        .into_iter()
        .zip(col2)
        .map(|(c1, c2)| c1.abs_diff(c2))
        .sum::<u64>();

    println!("Day 1\nPart I: {}", p1_result);
}

fn part2() {
    let input = super::get_input("../input/input_day1.txt");

    let mut col1 = vec![];
    //let mut col2 = vec![];
    let mut c2_freqs = HashMap::new();

    input.lines().for_each(|line| {
        let mut k = line.split_ascii_whitespace();
        let c1 = k.next().unwrap().parse::<u64>().unwrap();
        let c2 = k.next().unwrap().parse::<u64>().unwrap();
        c2_freqs.entry(c2).and_modify(|x| *x += 1).or_insert(1u64);
        col1.push(c1);
        //col2.push(c2);
    });

    let p2_result = col1
        .iter()
        .filter_map(|c1| c2_freqs.get_key_value(c1))
        .map(|(id, times)| id * times)
        .sum::<u64>();

    println!("Day 1\nPart II: {}", p2_result);
}
