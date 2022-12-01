use std::fs;

fn main() {
    let input = fs::read_to_string("./input/01.txt").unwrap();
    println!("p1: {}", p1(&input));
    println!("p2: {}", p2(&input));
}

fn p1(input: &str) -> String {
    let result = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    result.to_string()
}

fn p2(input: &str) -> String {
    let mut result = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    result.sort_by(|a, b| b.cmp(a));
    let sum: u32 = result.iter().take(3).sum();
    sum.to_string()
}

#[cfg(test)]
mod d01 {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn t1() {
        let result = p1(INPUT);
        assert_eq!(result, "24000");
    }

    #[test]
    fn t2() {
        let result = p2(INPUT);
        assert_eq!(result, "45000");
    }
}
