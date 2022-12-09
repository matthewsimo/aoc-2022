use std::{collections::HashSet, fs, time::Instant};

const R: (i16, i16) = (1, 0);
const L: (i16, i16) = (-1, 0);
const U: (i16, i16) = (0, 1);
const D: (i16, i16) = (0, -1);

fn do_move(rope: &(i16, i16), dir: &str) -> (i16, i16) {
    match dir {
        "R" => (rope.0 + R.0, rope.1 + R.1),
        "L" => (rope.0 + L.0, rope.1 + L.1),
        "U" => (rope.0 + U.0, rope.1 + U.1),
        "D" => (rope.0 + D.0, rope.1 + D.1),
        _ => panic!("Invalid Direction Found"),
    }
}

fn get_tail_move(h: (i16, i16), t: (i16, i16)) -> (i16, i16) {
    let d_x = if h.0 == t.0 {
        0
    } else {
        (h.0 - t.0) / (h.0 - t.0).abs()
    };
    let d_y = if h.1 == t.1 {
        0
    } else {
        (h.1 - t.1) / (h.1 - t.1).abs()
    };

    return (d_x, d_y);
}

fn is_touching(head: (i16, i16), tail: (i16, i16)) -> bool {
    (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1
}

fn main() {
    let t = Instant::now();
    let input = fs::read_to_string("./input/09.txt").unwrap();
    println!("p1: {}", p1(&input));
    println!("Ran p1 in {:.4?}", t.elapsed());
    let t = Instant::now();
    println!("p2: {}", p2(&input));
    println!("Ran p2 in {:.4?}", t.elapsed());
}

fn p1(input: &str) -> String {
    let lines = aoc::parse_lines::<String>(input.to_string());
    let mut visited: HashSet<(i16, i16)> = HashSet::new();
    let mut head: (i16, i16) = (0, 0);
    let mut tail: (i16, i16) = (0, 0);
    visited.insert(tail);

    for line in lines {
        let (dir, amount_str) = line.split_once(" ").expect("Incorrect input line");
        let amount: u8 = amount_str.parse().expect("Amount not a number");

        for _ in 0..amount {
            head = do_move(&head, dir);
            if !is_touching(head, tail) {
                let (d_x, d_y) = get_tail_move(head, tail);
                tail = (tail.0 + d_x, tail.1 + d_y);
            }

            visited.insert(tail);
        }
    }

    visited.len().to_string()
}

fn p2(input: &str) -> String {
    let lines = aoc::parse_lines::<String>(input.to_string());
    let mut visited: HashSet<(i16, i16)> = HashSet::new();
    let mut knots: [(i16, i16); 10] = [(0, 0); 10];
    visited.insert(knots[knots.len() - 1]);

    for line in lines {
        let (dir, amount_str) = line.split_once(" ").expect("Incorrect input line");
        let amount: u8 = amount_str.parse().expect("Amount not a number");

        for _ in 0..amount {
            knots[0] = do_move(&knots[0], dir);

            for j in 1..10 {
                let h = knots[j - 1];
                let t = knots[j];
                if !is_touching(h, t) {
                    let (d_x, d_y) = get_tail_move(h, t);
                    knots[j] = (t.0 + d_x, t.1 + d_y);
                }
            }

            visited.insert(knots[knots.len() - 1]);
        }
    }

    visited.len().to_string()
}

#[cfg(test)]
mod d09 {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn t1() {
        let result = p1(INPUT);
        assert_eq!(result, "13");
    }

    #[test]
    fn t2() {
        let result = p2(INPUT);
        assert_eq!(result, "1");

        let result = p2(INPUT2);
        assert_eq!(result, "36");
    }
}
