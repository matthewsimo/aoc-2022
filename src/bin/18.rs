use std::{collections::HashSet, fs, time::Instant};

fn parse(input: &str) -> HashSet<(i32, i32, i32)> {
    let mut blocks: HashSet<(i32, i32, i32)> = HashSet::new();

    let lines = input.split("\n");
    for line in lines {
        let l: Vec<i32> = line.split(",").map(|c| c.parse::<i32>().unwrap()).collect();
        blocks.insert((l[0], l[1], l[2]));
    }

    blocks
}

fn main() {
    let t = Instant::now();
    let input = fs::read_to_string("./input/18.txt").unwrap();
    println!("p1: {}", p1(&input));
    println!("Ran p1 in {:.4?}", t.elapsed());
    let t = Instant::now();
    println!("p2: {}", p2(&input));
    println!("Ran p2 in {:.4?}", t.elapsed());
}

fn p1(input: &str) -> String {
    let blocks = parse(input);
    let mut total_faces = 0;

    for block in &blocks {
        let (x, y, z) = block;
        let mut visible_faces = 0;

        if !blocks.contains(&(x - 1, *y, *z)) {
            visible_faces += 1;
        };
        if !blocks.contains(&(*x + 1, *y, *z)) {
            visible_faces += 1;
        };
        if !blocks.contains(&(*x, y - 1, *z)) {
            visible_faces += 1;
        };
        if !blocks.contains(&(*x, y + 1, *z)) {
            visible_faces += 1;
        };
        if !blocks.contains(&(*x, *y, z - 1)) {
            visible_faces += 1;
        };
        if !blocks.contains(&(*x, *y, z + 1)) {
            visible_faces += 1;
        };

        total_faces += visible_faces;
    }

    println!("total_faces: {:?}", total_faces);

    total_faces.to_string()
}

fn p2(input: &str) -> String {
    "bar".to_string()
}

#[cfg(test)]
mod d18 {
    use super::*;

    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn t1() {
        let result = p1(INPUT);
        assert_eq!(result, "64");
    }

    #[test]
    fn t2() {
        let result = p2(INPUT);
        assert_eq!(result, "bar");
    }
}
