#!/bin/bash

PATH=src/bin/$1.rs


echo "creating input file for day ${1}";
echo "" > input/$1.txt

echo "creating source file for day ${1}";
echo "use std::fs;

fn main() {
    let input = fs::read_to_string(\"./input/${1}.txt\").unwrap();
    println!(\"p1: {}\", p1(&input));
    println!(\"p2: {}\", p2(&input));
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
