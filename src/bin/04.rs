use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("./input/04.txt").unwrap();
    println!("p1: {}", p1(&input));
    println!("p2: {}", p2(&input));
}

fn p1(input: &str) -> String {
    let mut sum = 0;
    let lines = aoc::parse_lines::<String>(String::from(input));
    for line in lines {
        let (elf_a, elf_b) = line.split_once(",").expect("Line missing ','");
        let (sa_s, sa_e) = elf_a.split_once("-").expect("Bad input");
        let (sb_s, sb_e) = elf_b.split_once("-").expect("Bad input");

        let a_s: u16 = sa_s.parse().expect("input is a number");
        let a_e: u16 = sa_e.parse().expect("input is a number");
        let b_s: u16 = sb_s.parse().expect("input is a number");
        let b_e: u16 = sb_e.parse().expect("input is a number");

        if a_s <= b_s && a_e >= b_e {
            sum += 1;
        } else if b_s <= a_s && b_e >= a_e {
            sum += 1;
        }
    }

    sum.to_string()
}

fn p2(input: &str) -> String {
    let mut sum: i32 = 0;

    let lines = aoc::parse_lines::<String>(String::from(input));
    for line in lines {
        let (elf_a, elf_b) = line.split_once(",").expect("Line missing ','");
        let (a_s, a_e) = elf_a.split_once("-").expect("Bad input");
        let (b_s, b_e) = elf_b.split_once("-").expect("Bad input");

        let elf_a: HashSet<i32> = (a_s.parse().unwrap()..=a_e.parse().unwrap())
            .into_iter()
            .collect();
        let elf_b: HashSet<i32> = (b_s.parse().unwrap()..=b_e.parse().unwrap())
            .into_iter()
            .collect();

        let intersections_count = elf_a.intersection(&elf_b).count();

        if intersections_count > 0 {
            sum += 1;
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod d04 {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn t1() {
        let result = p1(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn t2() {
        let result = p2(INPUT);
        assert_eq!(result, "4");
    }
}
