use std::fs;

fn main() {
    let input = fs::read_to_string("./input/05.txt").unwrap();

    /*
                        [Q]     [P] [P]
                    [G] [V] [S] [Z] [F]
                [W] [V] [F] [Z] [W] [Q]
            [V] [T] [N] [J] [W] [B] [W]
        [Z] [L] [V] [B] [C] [R] [N] [M]
    [C] [W] [R] [H] [H] [P] [T] [M] [B]
    [Q] [Q] [M] [Z] [Z] [N] [G] [G] [J]
    [B] [R] [B] [C] [D] [H] [D] [C] [N]
     1   2   3   4   5   6   7   8   9
    */

    let stacks = vec![
        vec!["B", "Q", "C"],
        vec!["R", "Q", "W", "Z"],
        vec!["B", "M", "R", "L", "V"],
        vec!["C", "Z", "H", "V", "T", "W"],
        vec!["D", "Z", "H", "B", "N", "V", "G"],
        vec!["H", "N", "P", "C", "J", "F", "V", "Q"],
        vec!["D", "G", "T", "R", "W", "Z", "S"],
        vec!["C", "G", "M", "N", "B", "W", "Z", "P"],
        vec!["N", "J", "B", "M", "W", "Q", "F", "P"],
    ];

    println!("p1: {}", p1(&stacks, &input));
    println!("p2: {}", p2(&stacks, &input));
}

fn p1(stacks: &Vec<Vec<&str>>, input: &str) -> String {
    let lines: Vec<String> = aoc::parse_lines(String::from(input));
    let mut s = stacks.clone();

    for line in lines {
        let l = line.split(" ").collect::<Vec<&str>>();
        let (n, from, to) = (
            l[1].parse::<usize>().unwrap(),     // Move N
            l[3].parse::<usize>().unwrap() - 1, // From
            l[5].parse::<usize>().unwrap() - 1, // To
        );
        for _i in 1..=n {
            let p = s[from].pop().unwrap();
            s[to].push(p);
        }
    }

    let mut lasts = String::new();
    for v in s {
        lasts.push_str(v[v.len() - 1]);
    }

    lasts.to_string()
}

fn p2(stacks: &Vec<Vec<&str>>, input: &str) -> String {
    let lines: Vec<String> = aoc::parse_lines(String::from(input));
    let mut s = stacks.clone();

    for line in lines {
        let l = line.split(" ").collect::<Vec<&str>>();
        let (n, from, to) = (
            l[1].parse::<usize>().unwrap(),     // Move N
            l[3].parse::<usize>().unwrap() - 1, // From
            l[5].parse::<usize>().unwrap() - 1, // To
        );
        let take_index = s[from].len() - n;
        let mut tail = s[from].split_off(take_index);
        s[to].append(&mut tail);
    }

    let mut lasts = String::new();
    for v in s {
        lasts.push_str(v[v.len() - 1]);
    }

    lasts.to_string()
}

#[cfg(test)]
mod d05 {
    use super::*;

    const INPUT: &str = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn t1() {
        let stacks = vec![vec!["Z", "N"], vec!["M", "C", "D"], vec!["P"]];
        let result = p1(&stacks, INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn t2() {
        let stacks = vec![vec!["Z", "N"], vec!["M", "C", "D"], vec!["P"]];
        let result = p2(&stacks, INPUT);
        assert_eq!(result, "MCD");
    }
}
