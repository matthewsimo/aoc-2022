use std::{
    collections::{HashSet, VecDeque},
    fs,
    time::Instant,
};

fn get_grid(input: &str) -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .enumerate()
        .map(|(r, row)| {
            // println!("{:?}", l);
            row.chars()
                .enumerate()
                .map(|(c, ch)| {
                    if ch == 'S' {
                        start.0 = r;
                        start.1 = c;
                    }

                    if ch == 'E' {
                        end.0 = r;
                        end.1 = c;
                    }
                    ch
                })
                .collect()
        })
        .collect();
    (grid, start, end)
}

fn get_adjacent(pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut adjacent: Vec<(usize, usize)> = vec![];

    adjacent.push((pos.0 + 1, pos.1));
    adjacent.push((pos.0, pos.1 + 1));

    if pos.0 > 0 {
        adjacent.push((pos.0 - 1, pos.1));
    }

    if pos.1 > 0 {
        adjacent.push((pos.0, pos.1 - 1));
    }

    adjacent
}

fn is_bad_move(a: char, b: char) -> bool {
    (a as i32 - b as i32) > 1
}

fn is_bad_move_rev(a: char, b: char) -> bool {
    (a as i32 - b as i32) < -1
}

fn main() {
    let t = Instant::now();
    let input = fs::read_to_string("./input/12.txt").unwrap();
    println!("p1: {}", p1(&input));
    println!("Ran p1 in {:.4?}", t.elapsed());
    let t = Instant::now();
    println!("p2: {}", p2(&input));
    println!("Ran p2 in {:.4?}", t.elapsed());
}

fn p1(input: &str) -> String {
    let (mut grid, start, end) = get_grid(input);
    // Make S & E their actual char values
    grid[start.0][start.1] = 'a';
    grid[end.0][end.1] = 'z';

    let mut visited: HashSet<(usize, usize)> = HashSet::from([start]);
    let mut queue: VecDeque<(usize, (usize, usize))> = VecDeque::from(vec![(0, start)]);
    let mut move_count = 0;

    'top: while queue.len() > 0 {
        let (dist, pos) = queue.pop_front().expect("Popped empty queue");
        let maybe_next_pos = get_adjacent(pos);

        for next_pos in maybe_next_pos {
            // if this pos is off grid, skip
            if next_pos.0 >= grid.len() || next_pos.1 >= grid[0].len() {
                continue;
            }

            // if already visited, skip
            if visited.contains(&next_pos) {
                continue;
            }

            // if invalid move, skip
            if is_bad_move(grid[next_pos.0][next_pos.1], grid[pos.0][pos.1]) {
                continue;
            }

            // if we found end, exit
            if next_pos.0 == end.0 && next_pos.1 == end.1 {
                move_count = dist + 1;
                break 'top;
            }
            visited.insert(next_pos);
            queue.push_back((dist + 1, next_pos));
        }
    }

    move_count.to_string()
}

fn p2(input: &str) -> String {
    let (mut grid, start, end) = get_grid(input);
    // Make S & E their actual char values
    grid[start.0][start.1] = 'a';
    grid[end.0][end.1] = 'z';

    let mut visited: HashSet<(usize, usize)> = HashSet::from([end]);
    let mut queue: VecDeque<(usize, (usize, usize))> = VecDeque::from(vec![(0, end)]);
    let mut move_count = 0;

    'top: while queue.len() > 0 {
        let (dist, pos) = queue.pop_front().expect("Popped empty queue");
        let maybe_next_pos = get_adjacent(pos);

        for next_pos in maybe_next_pos {
            // if this pos is off grid, skip
            if next_pos.0 >= grid.len() || next_pos.1 >= grid[0].len() {
                continue;
            }

            // if already visited, skip
            if visited.contains(&next_pos) {
                continue;
            }

            // if invalid move, skip
            if is_bad_move_rev(grid[next_pos.0][next_pos.1], grid[pos.0][pos.1]) {
                continue;
            }

            // if we found end, exit
            if grid[next_pos.0][next_pos.1] == 'a' {
                move_count = dist + 1;
                break 'top;
            }

            visited.insert(next_pos);
            queue.push_back((dist + 1, next_pos));
        }
    }

    move_count.to_string()
}

#[cfg(test)]
mod d12 {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn t1() {
        let result = p1(INPUT);
        assert_eq!(result, "31");
    }

    #[test]
    fn t2() {
        let result = p2(INPUT);
        assert_eq!(result, "29");
    }
}
