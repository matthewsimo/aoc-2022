use std::{fs, time::Instant};

fn get_rock(round: usize) -> (Vec<(i32, i32)>, (i32, i32), i32) {
    // (points, start offset, width)
    match round % 5 {
        /*
        #### */
        0 => (vec![(0, 0), (1, 0), (2, 0), (3, 0)], (0, 0), 4),
        /*
         #
        ###
         # */
        1 => (vec![(0, 0), (1, 0), (2, 0), (1, 1), (1, -1)], (0, -1), 3),
        /*
          #
          #
        ### */
        2 => (vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], (0, 0), 3),
        /*
        #
        #
        #
        # */
        3 => (vec![(0, 0), (0, 1), (0, 2), (0, 3)], (0, 0), 1),
        /*
        ##
        ## */
        4 => (vec![(0, 0), (0, 1), (1, 0), (1, 1)], (0, 0), 2),
        _ => panic!("Incorrect round value received"),
    }
}

fn main() {
    let t = Instant::now();
    let input = fs::read_to_string("./input/17.txt").unwrap();
    println!("p1: {}", p1(&input));
    println!("Ran p1 in {:.4?}", t.elapsed());
    let t = Instant::now();
    println!("p2: {}", p2(&input));
    println!("Ran p2 in {:.4?}", t.elapsed());
}

fn p1(input: &str) -> String {
    let max_rounds = 10;
    let jets: Vec<char> = input.chars().collect();
    let mut jet_counter: usize = 0;
    let mut max_height: i32 = 0;

    println!("jets: {:?}", jets);

    for round in 0..max_rounds {
        let (rock, offset, width) = get_rock(round);
        let mut rock_counter: i32 = 0;
        let mut is_stopped = false;
        let mut pos = (2, 3 + max_height);

        while !is_stopped {
            println!("i: {:?} - pos:{:?}", rock_counter, pos);
            match rock_counter % 2 {
                0 => {
                    // Handle Jets
                    let i = jet_counter % jets.len();
                    let jet = jets[i];
                    match jet {
                        '<' => {
                            if pos.0 > 0 {
                                pos.0 -= 1;
                            }
                        }
                        '>' => {
                            if (pos.0 + width) < 7 {
                                pos.0 += 1;
                            }
                        }
                        _ => panic!("Invalid jet direction"),
                    };

                    jet_counter += 1;
                }
                1 => {
                    // Handle Move Down
                    println!("move down");
                    pos.1 -= 1;
                }
                _ => panic!("Unknown issue with rock_counter"),
            };

            rock_counter += 1;

            if rock_counter >= 5 {
                is_stopped = true;
            }
        }

        // set new max_height
        //
    }

    "foo".to_string()
}

fn p2(input: &str) -> String {
    "bar".to_string()
}

#[cfg(test)]
mod d17 {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn t1() {
        let result = p1(INPUT);
        assert_eq!(result, "3068");
    }

    #[test]
    fn t2() {
        let result = p2(INPUT);
        assert_eq!(result, "bar");
    }
}
