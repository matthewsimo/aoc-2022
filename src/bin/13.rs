use std::{fs, time::Instant};

fn main() {
    let t = Instant::now();
    let input = fs::read_to_string("./input/13.txt").unwrap();
    println!("p1: {}", p1(&input));
    println!("Ran p1 in {:.4?}", t.elapsed());
    let t = Instant::now();
    println!("p2: {}", p2(&input));
    println!("Ran p2 in {:.4?}", t.elapsed());
}

fn p1(_input: &str) -> String {
    "JSON is dumb".to_string()
}

fn p2(_input: &str) -> String {
    "JSON is dumb".to_string()
}

#[cfg(test)]
mod d13 {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn t1() {
        let result = p1(INPUT);
        assert_eq!(result, "JSON is dumb");
    }

    #[test]
    fn t2() {
        let result = p2(INPUT);
        assert_eq!(result, "JSON is dumb");
    }
}
