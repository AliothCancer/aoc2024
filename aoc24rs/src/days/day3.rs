//#![allow(unused, clippy::diverging_sub_expression, clippy::let_unit_value)]

use itertools::Itertools;
use sliding_windows::{IterExt, Storage};
use std::{collections::HashMap, time::Instant, vec};

use crate::days::get_input;

fn count_frequencies(vec: Vec<u64>) -> HashMap<u64, usize> {
    let mut frequency_map = HashMap::new();

    for &num in &vec {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    frequency_map
}
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

const INPUT_PATH: &str = "../input/input_day3.txt";
const TEST_INPUT: &str =
    "xmul(2,4)%mul((34,))&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
fn part1() {
    let test1 = "mmmul(1,1)mmul(2,2)mulmul(3mul(2,3)    mul(3,3)mul(4,4)";
    let input = get_input(INPUT_PATH);
    let mut tokens = input.chars().map(Token::to_token).peekable();
    //println!("{}", test1);
    let mut total_mul_add = 0;
    let mut mul_found = 0;
    while let Some(token) = tokens.find(|x| x == &Token::M) {
        //dbg!(token);
        while tokens.peek() == Some(&Token::M) {
            tokens.next();
        }
        let mut first_number = String::new();
        let mut second_number = String::new();

        match (
            tokens.next_if(|x| x == &Token::U),
            tokens.next_if(|x| x == &Token::L),
            tokens.next_if(|x| x == &Token::LeftPar),
        ) {
            (Some(Token::U), Some(Token::L), Some(Token::LeftPar)) => {
                // Raccogli il primo numero
                while matches!(tokens.peek(), Some(Token::Digit(k))) {
                    if let Some(Token::Digit(dig)) = tokens.next() {
                        if first_number.len() < 3 {
                            first_number.push(dig);
                        } else {
                            println!("dig dropped: {dig}");
                            break;
                        }
                    }
                }

                // Controlla che il prossimo token sia un Comma
                if matches!(tokens.peek(), Some(Token::Comma)) {
                    tokens.next(); // Consuma il Comma

                    // Raccogli il secondo numero
                    while matches!(tokens.peek(), Some(Token::Digit(_))) {
                        if let Some(Token::Digit(dig)) = tokens.next() {
                            if second_number.len() < 3 {
                                second_number.push(dig);
                            } else {
                                println!("dig dropped: {dig}");
                                break;
                            }
                        }
                    }

                    // Controlla la chiusura
                    if matches!(tokens.next(), Some(Token::RightPar)) {
                        let n1 = first_number.parse::<u64>().unwrap();
                        let n2 = second_number.parse::<u64>().unwrap();
                        //println!("mul({},{})", first_number, second_number);
                        total_mul_add += n1 * n2;
                        mul_found += 1;
                    }
                }
            }
            _ => continue, // Salta pattern non validi
        }
    }
    println!("total: {}\ncase found: {}", total_mul_add, mul_found);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Token {
    M,
    U,
    L,
    Digit(char),
    LeftPar,
    Comma,
    RightPar,
    D,
    O,
    N,
    Apos,
    T,
    Space,
    Ignore,
}

impl Token {
    fn to_token(ch: char) -> Token {
        match ch {
            'm' => Token::M,
            'u' => Token::U,
            'l' => Token::L,
            '(' => Token::LeftPar,
            ',' => Token::Comma,
            ')' => Token::RightPar,
            ' ' => Token::Space,
            ch if ch.is_numeric() => Token::Digit(ch),
            'd' => Token::D,
            'o' => Token::O,
            'n' => Token::N,
            '\'' => Token::Apos,
            't' => Token::T,
            _ => Token::Ignore,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MulToken {
    Mul,
    LeftPar,
    FirstNumber(u64),
    SecondNumber(u64),
    RightPar,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MulState {
    Do,
    Dont,
}

fn part2() {
    let test1 = "do()xmul(2,4)&mul[3,7]!^don't()_mmul(5567,5)+mulmul(32,64](mul(11,8)undo()?mul(8,5))";
    let input = get_input(INPUT_PATH);
    let mut tokens = input.chars().map(Token::to_token).peekable();
    //println!("{}", test1);
    //println!("{:?}", tokens.clone().collect_vec());
    let mut total_mul_add = 0;
    let mut mul_found = 0;
    let mut mul_executed = 0;
    let mut mul_enabled = true;
    while let Some(token) = tokens.find(|x| x == &Token::M || x == &Token::D) {
    
        while tokens.peek() == Some(&Token::M) {
            tokens.next();
        }
        while tokens.peek() == Some(&Token::D) {
            tokens.next();
        }

        if let Some(Token::O) = tokens.next_if_eq(&Token::O) {
            match (tokens.next_if(|x| x == &Token::N || x == &Token::LeftPar)) {
                Some(Token::LeftPar) => match tokens.next_if_eq(&Token::RightPar) {
                    Some(Token::RightPar) => {
                        mul_enabled = true;
                        //println!("found match 'do()' mul enabled");
                    },
                    _ => continue
                },
                Some(Token::N) => if let (
                        Some(Token::Apos),
                        Some(Token::T),
                        Some(Token::LeftPar),
                        Some(Token::RightPar),
                    ) = (
                    tokens.next_if_eq(&Token::Apos),
                    tokens.next_if_eq(&Token::T),
                    tokens.next_if_eq(&Token::LeftPar),
                    tokens.next_if_eq(&Token::RightPar)
                ) {
                    mul_enabled = false;
                    //println!("found match 'don't()' mul disabled");
                },
                
                _ => (),
            }
        }

        let mut first_number = String::new();
        let mut second_number = String::new();

        match (
            tokens.next_if_eq(&Token::U),
            tokens.next_if_eq(&Token::L),
            tokens.next_if_eq(&Token::LeftPar),
        ) {
            (Some(Token::U), Some(Token::L), Some(Token::LeftPar)) => {
                // Raccogli il primo numero
                while matches!(tokens.peek(), Some(Token::Digit(k))) {
                    if let Some(Token::Digit(dig)) = tokens.next() {
                        if first_number.len() < 3 {
                            first_number.push(dig);
                        } else {
                            println!("dig dropped: {dig}");
                            break;
                        }
                    }
                }

                // Controlla che il prossimo token sia un Comma
                if matches!(tokens.peek(), Some(Token::Comma)) {
                    tokens.next(); // Consuma il Comma

                    // Raccogli il secondo numero
                    while matches!(tokens.peek(), Some(Token::Digit(_))) {
                        if let Some(Token::Digit(dig)) = tokens.next() {
                            if second_number.len() < 3 {
                                second_number.push(dig);
                            } else {
                                println!("dig dropped: {dig}");
                                break;
                            }
                        }
                    }

                    // Controlla la chiusura
                    if matches!(tokens.next(), Some(Token::RightPar)) {
                        let n1 = first_number.parse::<u64>().unwrap();
                        let n2 = second_number.parse::<u64>().unwrap();
                        if mul_enabled {
                            //println!("mul({},{})", first_number, second_number);
                            total_mul_add += n1 * n2;
                            mul_executed += 1;
                        }
                        mul_found += 1;
                    }
                }
            }
            _ => continue, // Salta pattern non validi
        }
    }
    println!("\n\nPART II\ntotal: {}\nmul found: {}\nmul executed: {}", total_mul_add, mul_found, mul_executed);
}
