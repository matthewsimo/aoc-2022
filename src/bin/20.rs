use std::{
    collections::{HashSet, LinkedList, VecDeque},
    fs,
    time::Instant,
};

fn main() {
    let t = Instant::now();
    let input = fs::read_to_string("./input/20.txt").unwrap();
    println!("p1: {}", p1(&input));
    println!("Ran p1 in {:.4?}", t.elapsed());
    let t = Instant::now();
    println!("p2: {}", p2(&input));
    println!("Ran p2 in {:.4?}", t.elapsed());
}

fn p1(input: &str) -> String {
    let lines: Vec<i64> = input.split("\n").map(|l| l.parse().unwrap()).collect();
    let mut decrypt: LinkedList<i64> = LinkedList::from(lines.clone());

    println!("l: {:?}", lines.len());
    // println!(": {:?}", decrypt);
    for i in 0..lines.len() {
        println!("move: {:?}", lines[i]);

        let pos = lines[i] % lines.len() as i64;
        println!("pos: {:?}", pos);

        // decrypt.remove(i);
        // decrypt.insert(pos as usize, lines[i]);

        // println!(": {:?}", decrypt);
    }

    // println!("{:?} - {:?}", 1000, lines[1000 % lines.len()]);

    let a = decrypt[1000 % lines.len()];
    let b = decrypt[2000 % lines.len()];
    let c = decrypt[3000 % lines.len()];
    let sum = a + b + c;
    println!("sum: {:?}", sum);

    "0".to_string()
}

fn p2(input: &str) -> String {
    "bar".to_string()
}

#[cfg(test)]
mod d20 {
    use super::*;

    const INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn t1() {
        let result = p1(INPUT);
        assert_eq!(result, "3");
    }

    #[test]
    fn t2() {
        let result = p2(INPUT);
        assert_eq!(result, "bar");
    }
}
