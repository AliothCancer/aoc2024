use std::fs;

pub mod day1;
pub mod day2;

fn get_input(path: &str) -> String{
    fs::read_to_string(path)
    .expect("error getting the input")
}