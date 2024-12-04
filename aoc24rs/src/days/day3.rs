//#![allow(unused, clippy::diverging_sub_expression, clippy::let_unit_value)]

use itertools::Itertools;
use sliding_windows::{IterExt, Storage};
use std::{time::Instant, vec};

use crate::days::get_input;

pub fn run() {
    let i = Instant::now();
    part1();
    let duration = i.elapsed();
    println!("{} seconds", duration.as_nanos() as f64 / 1_000_000_000.0);

    //let i = Instant::now();
    //part2();
    //let duration = i.elapsed();
    //println!("{} seconds", duration.as_nanos() as f64 / 1_000_000_000.0);
}

const INPUT_PATH: &str = "../input/input_day3.txt";
const TEST_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
fn part1() {
    let input = get_input(INPUT_PATH);
    let mut tokens = TEST_INPUT.chars().map(Token::to_token).peekable();
    println!("{}",TEST_INPUT);
    //let mut tokens = input.chars().map(Token::to_token).peekable();

    let mut total_mul = 0;
    while let Some(token) = tokens.find(|x| x == &Token::M) {
        //dbg!(token);
        let mut first_number = String::new();
        let mut second_number = String::new();

        match (tokens.next(), tokens.next(), tokens.next()) {
            (Some(Token::U), Some(Token::L), Some(Token::LeftPar)) => {
                // Raccogli il primo numero
                while matches!(tokens.peek(), Some(Token::Digit(k))) {
                    if let Some(Token::Digit(dig)) = tokens.next() {
                        if first_number.len() < 3 {
                            first_number.push(dig);
                        } else {
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
                                break;
                            }
                        }
                    }

                    // Controlla la chiusura
                    if matches!(tokens.next(), Some(Token::RightPar)) {
                        let n1 = first_number.parse::<u64>().unwrap();
                        let n2 = second_number.parse::<u64>().unwrap();
                        println!("Found mul({}, {})", first_number, second_number);
                        total_mul += n1 * n2;
                    }
                }
            }
            _ => continue, // Salta pattern non validi
        }
    }
    println!("total: {}", total_mul);
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
            ch if ch.is_numeric() => Token::Digit(ch),
            _ => Token::Ignore,
        }
    }
}

fn part2() {
    let input = super::get_input(INPUT_PATH);

    let result = unimplemented!();
    println!("result part II: {:?}", result);
}
