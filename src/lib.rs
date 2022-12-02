use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn parse_lines<T>(input: String) -> Vec<T>
where
    T: Debug,
    T: FromStr,
{
    let lines = input
        .split("\n")
        .filter_map(|line| Some(line.parse::<T>().ok().expect("Parsing Line Failed")))
        .collect::<Vec<T>>();
    return lines;
}
