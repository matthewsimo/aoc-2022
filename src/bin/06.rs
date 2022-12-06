use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("./input/06.txt").unwrap();
    println!("p1: {}", p1(&input));
    println!("p2: {}", p2(&input));
}

fn yeet(input: &str, space: usize) -> usize {
    let mut i = 0;
    loop {
        let subset = &input[i..(i + space)];
        let hash: HashSet<char> = subset.chars().collect();
        match hash.len() == space {
            true => break,
            false => i += 1,
        }
    }

    i + space
}

fn p1(input: &str) -> String {
    let marker = yeet(input, 4);
    (marker).to_string()
}

fn p2(input: &str) -> String {
    let marker = yeet(input, 14);
    (marker).to_string()
}

#[cfg(test)]
mod d06 {
    use super::*;

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn t1() {
        let result = p1(INPUT);
        assert_eq!(result, "7");
    }

    #[test]
    fn t2() {
        let result = p2(INPUT);
        assert_eq!(result, "19");
    }
}
