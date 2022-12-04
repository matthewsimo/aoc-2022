use std::fs;

fn main() {
    let input = fs::read_to_string("./input/03.txt").unwrap();
    println!("p1: {}", p1(&input));
    println!("p2: {}", p2(&input));
}

fn p1(input: &str) -> String {
    let lines = aoc::parse_lines::<String>(input.to_string());

    let sum = lines
        .iter()
        .map(|bag| {
            let midpoint = bag.len() / 2;
            let compartment_one = &bag[0..midpoint];
            let compartment_two = &bag[midpoint..bag.len()];
            let shared_char = compartment_one
                .chars()
                .find(|c| compartment_two.contains(*c))
                .expect("No shared letter somehow");

            score(shared_char)
        })
        .sum::<u32>();

    sum.to_string()
}

fn p2(input: &str) -> String {
    let mut sum: u32 = 0;
    let lines = aoc::parse_lines::<String>(input.to_string());
    let teams: Vec<Vec<String>> = lines.chunks(3).map(|x| x.to_vec()).collect();

    for team in teams {
        let shared_char = team[0]
            .chars()
            .find(|c| team[1].contains(*c) && team[2].contains(*c))
            .expect("No shared letter somehow");
        sum = sum + score(shared_char);
    }

    sum.to_string()
}

fn score(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u8 - b'a' + 1) as u32,
        'A'..='Z' => (c as u8 - b'A' + 27) as u32,
        _ => panic!("Invalid ASCII Char"),
    }
}

#[cfg(test)]
mod d03 {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn t1() {
        let result = p1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn t2() {
        let result = p2(INPUT);
        assert_eq!(result, "70");
    }
}
