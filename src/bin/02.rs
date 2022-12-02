use std::str::FromStr;

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}
impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Invalid move".to_string()),
        }
    }
}

#[derive(Debug)]
struct Round(Move, Move);
impl FromStr for Round {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (o, m) = s.split_once(" ").unwrap();
        Ok(Round(o.parse::<Move>()?, m.parse::<Move>()?))
    }
}

fn main() {
    let input = aoc::read_input("./input/02.txt");
    println!("p1: {}", p1(&input));
    println!("p2: {}", p2(&input));
}

fn p1(input: &str) -> String {
    let lines = aoc::parse_lines::<Round>(String::from(input));

    let sum: i32 = lines
        .iter()
        .map(|r| {
            match r {
                Round(Move::Rock, response) => match response {
                    Move::Rock => 4,     // println!("Rock & Rock - 1 + 3"),
                    Move::Paper => 8,    //println!("Rock & Paper - 2 + 6"),
                    Move::Scissors => 3, //println!("Rock & Scissors - 3 + 0"),
                },
                Round(Move::Paper, response) => match response {
                    Move::Rock => 1,     //println!("Paper & Rock - 1 + 0"),
                    Move::Paper => 5,    //println!("Paper & Paper - 2 + 3"),
                    Move::Scissors => 9, //println!("Paper & Scissors - 3 + 6"),
                },
                Round(Move::Scissors, response) => match response {
                    Move::Rock => 7,     //println!("Scissors & Rock - 1 + 6"),
                    Move::Paper => 2,    //println!("Scissors & Paper - 2 + 0"),
                    Move::Scissors => 6, //println!("Scissors & Scissors - 3 + 3"),
                },
            }
        })
        .sum();

    sum.to_string()
}

fn p2(input: &str) -> String {
    let lines = aoc::parse_lines::<Round>(String::from(input));

    let sum: i32 = lines
        .iter()
        .map(|r| {
            match r {
                Round(Move::Rock, response) => match response {
                    Move::Rock => 3,     //println!("Rock & L: Scissors - 3 + 0"),
                    Move::Paper => 4,    //println!("Rock & D: Rock - 1 + 3"),
                    Move::Scissors => 8, //println!("Rock & W: Paper - 2 + 6"),
                },
                Round(Move::Paper, response) => match response {
                    Move::Rock => 1,     //println!("Paper & L: Rock - 1 + 0"),
                    Move::Paper => 5,    //println!("Paper & D: Paper - 2 + 3"),
                    Move::Scissors => 9, //println!("Paper & W: Scissors - 3 + 6"),
                },
                Round(Move::Scissors, response) => match response {
                    Move::Rock => 2,     //println!("Scissors & L: Paper - 2 + 0"),
                    Move::Paper => 6,    //println!("Scissors & D: Scissors - 3 + 3"),
                    Move::Scissors => 7, //println!("Scissors & W: Rock - 1 + 6"),
                },
            }
        })
        .sum();

    sum.to_string()
}

#[cfg(test)]
mod d02 {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn t1() {
        let result = p1(INPUT);
        assert_eq!(result, "15");
    }

    #[test]
    fn t2() {
        let result = p2(INPUT);
        assert_eq!(result, "12");
    }
}
