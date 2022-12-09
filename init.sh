#!/bin/bash

PATH=src/bin/$1.rs


# Download the input into proper place
/bin/bash ./get_input.sh $1;

# Create starter solution file
echo "creating source file for day ${1}";
echo "use std::{fs, time::Instant};

fn main() {
    let t = Instant::now();
    let input = fs::read_to_string(\"./input/${1}.txt\").unwrap();
    println!(\"p1: {}\", p1(&input));
    println!(\"Ran p1 in {:.4?}\", t.elapsed());
    let t = Instant::now();
    println!(\"p2: {}\", p2(&input));
    println!(\"Ran p2 in {:.4?}\", t.elapsed());
}

fn p1(input: &str) -> String {
  \"foo\".to_string()
}

fn p2(input: &str) -> String {
  \"bar\".to_string()
}

#[cfg(test)]
mod d${1} {
    use super::*;

    const INPUT: &str = \"\";

    #[test]
    fn t1() {
        let result = p1(INPUT);
        assert_eq!(result, \"foo\");
    }

    #[test]
    fn t2() {
        let result = p2(INPUT);
        assert_eq!(result, \"bar\");
    }
}
" >> $PATH
