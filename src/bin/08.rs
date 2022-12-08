use std::{fs, time::Instant};

fn get_trees(input: &str) -> Vec<Vec<u32>> {
    let lines = aoc::parse_lines::<String>(String::from(input));
    let mut trees: Vec<Vec<u32>> = vec![];
    for line in lines {
        let tree_line: Vec<u32> = line
            .chars()
            .map(|c| c.to_digit(10 as u32).expect("Invalid Digit"))
            .collect();
        trees.push(tree_line);
    }
    trees
}

fn can_see_edge(tree: &u32, trees: Vec<((usize, usize), u32)>) -> bool {
    trees.iter().all(|t| *tree > t.1)
}

fn get_view(tree: &u32, trees: Vec<((usize, usize), u32)>) -> u32 {
    let mut view: u32 = 0;

    for t in trees {
        match t {
            (_, h) if tree > &h => view += 1,
            _ => {
                view += 1;
                break;
            }
        }
    }

    view
}

fn main() {
    let t = Instant::now();
    let input = fs::read_to_string("./input/08.txt").unwrap();
    println!("p1: {}", p1(&input));
    println!("Ran p1 in {:.4?}", t.elapsed());
    let t = Instant::now();
    println!("p2: {}", p2(&input));
    println!("Ran p2 in {:.4?}", t.elapsed());
}

fn p1(input: &str) -> String {
    let trees = get_trees(input);
    let mut visible_trees: Vec<Vec<bool>> = vec![];

    let max_y = trees.len() - 1;
    let max_x = trees[0].len() - 1;
    for (y_index, tree_line) in trees.iter().enumerate() {
        // println!("y:{:?} - {:?}", y_index, tree_line);
        let visible_tree_line: Vec<bool> = match y_index {
            _ if y_index == max_y || y_index == 0 => vec![true; max_x + 1],
            _ => tree_line
                .iter()
                .enumerate()
                .map(|(x_index, tree)| {
                    match x_index {
                        _ if x_index == max_x || x_index == 0 => true,
                        _ => {
                            // println!("x,y: ({:?},{:?}) - t:{:?}", x_index, y_index, tree);
                            let north_trees = (0..y_index)
                                .map(|y_needle| ((x_index, y_needle), trees[y_needle][x_index]))
                                .collect::<Vec<((usize, usize), u32)>>();
                            if can_see_edge(tree, north_trees) {
                                return true;
                            };

                            let south_trees = (y_index + 1..max_y + 1)
                                .map(|y_needle| ((x_index, y_needle), trees[y_needle][x_index]))
                                .collect::<Vec<((usize, usize), u32)>>();
                            if can_see_edge(tree, south_trees) {
                                return true;
                            };

                            let west_trees = (0..x_index)
                                .map(|x_needle| ((x_needle, y_index), trees[y_index][x_needle]))
                                .collect::<Vec<((usize, usize), u32)>>();
                            if can_see_edge(tree, west_trees) {
                                return true;
                            };

                            let east_trees = (x_index + 1..max_x + 1)
                                .map(|x_needle| ((x_needle, y_index), trees[y_index][x_needle]))
                                .collect::<Vec<((usize, usize), u32)>>();
                            if can_see_edge(tree, east_trees) {
                                return true;
                            };

                            false
                        }
                    }
                })
                .collect(),
        };
        visible_trees.push(visible_tree_line);
    }

    // println!("{:?}", total_visible);

    let total_visible = visible_trees.iter().flatten().filter(|t| **t).count();

    total_visible.to_string()
}

fn p2(input: &str) -> String {
    let trees = get_trees(input);
    let mut trees_view_distance: Vec<Vec<((usize, usize), u32)>> = vec![];

    let max_y = trees.len() - 1;
    let max_x = trees[0].len() - 1;
    for (y_index, tree_line) in trees.iter().enumerate() {
        // println!("y:{:?} - {:?}", y_index, tree_line);
        let tree_line_view_distance: Vec<((usize, usize), u32)> = match y_index {
            _ => tree_line
                .iter()
                .enumerate()
                .map(|(x_index, tree)| match x_index {
                    _ if y_index == max_y || y_index == 0 || x_index == max_x || x_index == 0 => {
                        ((x_index, y_index), 0)
                    }
                    _ => {
                        // println!("x,y: ({:?},{:?}) - t:{:?}", x_index, y_index, tree);
                        let north_trees = (0..y_index)
                            .rev()
                            .map(|y_needle| ((x_index, y_needle), trees[y_needle][x_index]))
                            .collect::<Vec<((usize, usize), u32)>>();

                        let south_trees = (y_index + 1..max_y + 1)
                            .map(|y_needle| ((x_index, y_needle), trees[y_needle][x_index]))
                            .collect::<Vec<((usize, usize), u32)>>();

                        let west_trees = (0..x_index)
                            .rev()
                            .map(|x_needle| ((x_needle, y_index), trees[y_index][x_needle]))
                            .collect::<Vec<((usize, usize), u32)>>();

                        let east_trees = (x_index + 1..max_x + 1)
                            .map(|x_needle| ((x_needle, y_index), trees[y_index][x_needle]))
                            .collect::<Vec<((usize, usize), u32)>>();

                        let total_view: u32 = get_view(tree, north_trees)
                            * get_view(tree, south_trees)
                            * get_view(tree, west_trees)
                            * get_view(tree, east_trees);

                        // println!("total_view: {:?}", total_view);
                        ((x_index, y_index), total_view)
                    }
                })
                .collect(),
        };
        trees_view_distance.push(tree_line_view_distance);
    }

    // for tree_line in trees_view_distance {
    //     println!("{:?}", tree_line);
    // }

    let highest_score = trees_view_distance
        .iter()
        .flatten()
        .max_by(|x, y| x.1.cmp(&y.1))
        .expect("No High Score Found");

    highest_score.1.to_string()
}

#[cfg(test)]
mod d08 {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn t1() {
        let result = p1(INPUT);
        assert_eq!(result, "21");
    }

    #[test]
    fn t2() {
        let result = p2(INPUT);
        assert_eq!(result, "8");
    }
}
