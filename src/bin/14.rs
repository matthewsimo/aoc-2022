use std::{cmp, collections::HashSet, fs, time::Instant};

fn generate_rocks(input: &str) -> (HashSet<(i32, i32)>, i32) {
    let mut blocked: HashSet<(i32, i32)> = HashSet::new();
    let mut max_y: i32 = 0;
    let i: Vec<Vec<(i32, i32)>> = input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|c| {
                    let (x, y) = c.split_once(",").unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect()
        })
        .collect();

    for line in i {
        let _: Vec<_> = line
            .windows(2)
            .map(|pair| {
                let mut count = 0;
                let mut dir = true; // true -> x || false -> y
                let mut first = pair[0];

                if pair[0].0 != pair[1].0 {
                    count = (pair[0].0 - pair[1].0).abs();
                    if (pair[0].0 - pair[1].0) > 0 {
                        first = pair[1];
                    }
                } else if pair[0].1 != pair[1].1 {
                    count = (pair[0].1 - pair[1].1).abs();
                    if (pair[0].1 - pair[1].1) > 0 {
                        first = pair[1];
                    }
                    dir = false;
                }

                match dir {
                    true => {
                        // For horizontal
                        for i in 0..=count {
                            blocked.insert((first.0 + i, first.1));
                            max_y = cmp::max(max_y, first.1);
                        }
                    }
                    false => {
                        // For vertical
                        for i in 0..=count {
                            blocked.insert((first.0, first.1 + i));
                            max_y = cmp::max(max_y, first.1 + i);
                        }
                    }
                }
            })
            .collect();
    }

    (blocked, max_y)
}

fn main() {
    let t = Instant::now();
    let input = fs::read_to_string("./input/14.txt").unwrap();
    println!("p1: {}", p1(&input));
    println!("Ran p1 in {:.4?}", t.elapsed());
    let t = Instant::now();
    println!("p2: {}", p2(&input));
    println!("Ran p2 in {:.4?}", t.elapsed());
}

fn p1(input: &str) -> String {
    let (mut blocked, max_y) = generate_rocks(input);
    let mut sand_at_rest: i32 = 0;
    // Loop for each sand unit
    'outer: loop {
        let mut x: i32 = 500;
        let mut y: i32 = 0;
        // Loop until this sand unit is at rest
        loop {
            if y >= max_y {
                break 'outer;
            }
            // down
            if !blocked.contains(&(x, y + 1)) {
                y += 1;
                continue;
            }
            // left & down
            if !blocked.contains(&(x - 1, y + 1)) {
                y += 1;
                x -= 1;
                continue;
            }
            // right and down
            if !blocked.contains(&(x + 1, y + 1)) {
                y += 1;
                x += 1;
                continue;
            }

            // println!("{:?},{:?}", x, y);
            blocked.insert((x, y));
            sand_at_rest += 1;
            break;
        }
    }

    // println!("{:?}", sand_at_rest);
    sand_at_rest.to_string()
}

fn p2(input: &str) -> String {
    let (mut blocked, max_y) = generate_rocks(input);
    let mut sand_at_rest: i32 = 0;
    // Loop for each sand unit
    loop {
        if blocked.contains(&(500, 0)) {
            break;
        }
        let mut x: i32 = 500;
        let mut y: i32 = 0;
        // Loop until this sand unit is at rest
        loop {
            if y >= max_y + 1 {
                break;
            }
            // down
            if !blocked.contains(&(x, y + 1)) {
                y += 1;
                continue;
            }
            // left & down
            if !blocked.contains(&(x - 1, y + 1)) {
                y += 1;
                x -= 1;
                continue;
            }
            // right and down
            if !blocked.contains(&(x + 1, y + 1)) {
                y += 1;
                x += 1;
                continue;
            }

            break;
        }
        // println!("{:?},{:?}", x, y);
        blocked.insert((x, y));
        sand_at_rest += 1;
    }

    // println!("blocked:{:?}", blocked);
    // println!("max_y:{:?}", max_y);
    // println!("{:?}", sand_at_rest);
    sand_at_rest.to_string()
}

#[cfg(test)]
mod d14 {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn t1() {
        let result = p1(INPUT);
        assert_eq!(result, "24");
    }

    #[test]
    fn t2() {
        let result = p2(INPUT);
        assert_eq!(result, "93");
    }
}
