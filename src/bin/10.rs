use std::{fs, time::Instant};

fn main() {
    let t = Instant::now();
    let input = fs::read_to_string("./input/10.txt").unwrap();
    println!("p1: {}", p1(&input));
    println!("Ran p1 in {:.4?}", t.elapsed());
    let t = Instant::now();
    println!("p2: {}", p2(&input));
    println!("Ran p2 in {:.4?}", t.elapsed());
}

fn p1(input: &str) -> String {
    let mut x: i16 = 1;
    let mut x_list: Vec<i16> = vec![0];

    let lines = aoc::parse_lines::<String>(input.to_string());
    for line in lines {
        match line.as_str() {
            "noop" => x_list.push(x),
            _ => {
                let (_op, value) = line.split_once(" ").expect("Failed splitting line");
                x_list.push(x);
                x_list.push(x);
                x += value.parse::<i16>().expect("Failed parsing Add Value");
            }
        }
    }
    // push final x
    x_list.push(x);

    let mut signal_list: Vec<isize> = vec![];
    let _x_list2 = x_list
        .iter()
        .enumerate()
        .map(|(cycle, x)| {
            if cycle % 40 == 20 {
                signal_list.push((cycle * *x as usize) as isize);
            }
            (cycle as i16, x)
        })
        .collect::<Vec<(i16, &i16)>>();
    // println!("x_list2: {:?}", x_list2);
    // println!("signal_list: {:?}", signal_list);
    let sum: isize = signal_list.iter().sum();
    sum.to_string()
}

fn p2(input: &str) -> String {
    let mut screen: [[&str; 40]; 6] = [["."; 40]; 6];
    let mut x: i16 = 1;
    let mut x_list: Vec<i16> = vec![];

    let lines = aoc::parse_lines::<String>(input.to_string());
    for line in lines {
        match line.as_str() {
            "noop" => x_list.push(x),
            _ => {
                let (_op, value) = line.split_once(" ").expect("Failed splitting line");
                x_list.push(x);
                x_list.push(x);
                x += value.parse::<i16>().expect("Failed parsing Add Value");
            }
        }
    }

    let mut row: usize = 0;
    let mut col: usize = 0;
    for c in 1..=x_list.len() {
        if col == 40 {
            row += 1;
            col = 0;
        }
        let should_draw = (x_list[c - 1] - col as i16).abs() <= 1;
        if should_draw {
            screen[row][col] = "#";
        }
        col += 1;
    }

    // I can't read anything
    println!("{:?} ", screen[0].join("").replace(".", " "));
    println!("{:?} ", screen[1].join("").replace(".", " "));
    println!("{:?} ", screen[2].join("").replace(".", " "));
    println!("{:?} ", screen[3].join("").replace(".", " "));
    println!("{:?} ", screen[4].join("").replace(".", " "));
    println!("{:?} ", screen[5].join("").replace(".", " "));

    // Return for test
    let s = screen.map(|l| l.join("")).join("\n");
    s.to_string()
}

#[cfg(test)]
mod d10 {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn t1() {
        let result = p1(INPUT);
        assert_eq!(result, "13140");
    }

    #[test]
    fn t2() {
        let result = p2(INPUT);
        let str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(result, str);
    }
}
