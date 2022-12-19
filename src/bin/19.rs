use std::{fs, time::Instant};

fn main() {
    let t = Instant::now();
    let input = fs::read_to_string("./input/19.txt").unwrap();
    println!("p1: {}", p1(&input));
    println!("Ran p1 in {:.4?}", t.elapsed());
    let t = Instant::now();
    println!("p2: {}", p2(&input));
    println!("Ran p2 in {:.4?}", t.elapsed());
}

fn p1(input: &str) -> String {
    "foo".to_string()
}

fn p2(input: &str) -> String {
    "bar".to_string()
}

#[cfg(test)]
mod d19 {
    use super::*;

    const INPUT: &str = "Blueprint 1:
  Each ore robot costs 4 ore.
  Each clay robot costs 2 ore.
  Each obsidian robot costs 3 ore and 14 clay.
  Each geode robot costs 2 ore and 7 obsidian.

Blueprint 2:
  Each ore robot costs 2 ore.
  Each clay robot costs 3 ore.
  Each obsidian robot costs 3 ore and 8 clay.
  Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn t1() {
        let result = p1(INPUT);
        assert_eq!(result, "33");
    }

    #[test]
    fn t2() {
        let result = p2(INPUT);
        assert_eq!(result, "bar");
    }
}
