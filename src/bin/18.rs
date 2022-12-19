use std::{
    collections::{HashSet, VecDeque},
    fs,
    time::Instant,
};

type BlockData = (
    HashSet<(i32, i32, i32)>,
    ((i32, i32), (i32, i32), (i32, i32)),
);

fn parse(input: &str) -> BlockData {
    let mut blocks: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut min_x: Option<i32> = None;
    let mut max_x: Option<i32> = None;
    let mut min_y: Option<i32> = None;
    let mut max_y: Option<i32> = None;
    let mut min_z: Option<i32> = None;
    let mut max_z: Option<i32> = None;

    let lines = input.split("\n");
    for line in lines {
        let l: Vec<i32> = line.split(",").map(|c| c.parse::<i32>().unwrap()).collect();
        blocks.insert((l[0], l[1], l[2]));

        match min_x {
            None => min_x = Some(l[0]),
            Some(_) => {
                if min_x.unwrap() > l[0] {
                    min_x = Some(l[0])
                }
            }
        }

        match max_x {
            None => max_x = Some(l[0]),
            Some(_) => {
                if max_x.unwrap() < l[0] {
                    max_x = Some(l[0])
                }
            }
        }

        match min_y {
            None => min_y = Some(l[1]),
            Some(_) => {
                if min_y.unwrap() > l[1] {
                    min_y = Some(l[1])
                }
            }
        }

        match max_y {
            None => max_y = Some(l[1]),
            Some(_) => {
                if max_y.unwrap() < l[1] {
                    max_y = Some(l[1])
                }
            }
        }

        match min_z {
            None => min_z = Some(l[2]),
            Some(_) => {
                if min_z.unwrap() > l[2] {
                    min_z = Some(l[2])
                }
            }
        }

        match max_z {
            None => max_z = Some(l[2]),
            Some(_) => {
                if max_z.unwrap() < l[2] {
                    max_z = Some(l[2])
                }
            }
        }
    }

    let min_maxes = (
        (min_x.unwrap(), max_x.unwrap()),
        (min_y.unwrap(), max_y.unwrap()),
        (min_z.unwrap(), max_z.unwrap()),
    );
    (blocks, min_maxes)
}

const NEIGHBORS: [(i32, i32, i32); 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

fn is_external(x: i32, y: i32, z: i32, block_data: &BlockData) -> bool {
    let mut visited: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
    queue.push_back((x, y, z));

    let (blocks, min_max) = block_data;

    while queue.len() > 0 {
        let pos = queue.pop_front().unwrap();
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);

        if blocks.contains(&pos) {
            continue;
        }
        // less than mins or more than maxs return true
        if pos.0 < min_max.0 .0
            || pos.0 > min_max.0 .1
            || pos.1 < min_max.1 .0
            || pos.1 > min_max.1 .1
            || pos.2 < min_max.2 .0
            || pos.2 > min_max.2 .1
        {
            return true;
        }

        for (nx, ny, nz) in &NEIGHBORS {
            queue.push_back((pos.0 + nx, pos.1 + ny, pos.2 + nz));
        }
    }

    false
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
    let (blocks, _) = parse(input);
    let mut total_faces = 0;

    for (x, y, z) in &blocks {
        let mut visible_faces = 0;

        for (nx, ny, nz) in &NEIGHBORS {
            if !blocks.contains(&(x + nx, y + ny, z + nz)) {
                visible_faces += 1;
            };
        }

        total_faces += visible_faces;
    }

    total_faces.to_string()
}

fn p2(input: &str) -> String {
    let block_data = parse(input);
    let mut total_faces = 0;

    let blocks_copy = block_data.0.clone();

    for (x, y, z) in &blocks_copy {
        for (nx, ny, nz) in &NEIGHBORS {
            let is_external = is_external(x + nx, y + ny, z + nz, &block_data);
            if is_external {
                total_faces += 1;
            }
        }
    }

    total_faces.to_string()
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
        assert_eq!(result, "58");
    }
}
