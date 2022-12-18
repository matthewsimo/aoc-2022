use std::{collections::HashMap, fs, time::Instant};

fn parse(input: &str) -> HashMap<&str, (i32, Vec<&str>)> {
    let mut rooms = HashMap::new();

    //Valve AA has flow rate=0; tunnels lead to valves DD, II, BB

    let lines = input.split("\n");
    for line in lines {
        // println!("line:{:?}", line);
        let words: Vec<&str> = line.split(" ").collect();
        let room = words[1];
        let (_, raw_flow_rate) = words[4].split_once("=").unwrap();
        let flow_rate: i32 = (&raw_flow_rate[0..raw_flow_rate.len() - 1])
            .parse()
            .unwrap();

        let mut tunnels: Vec<&str> = vec![];
        for t in 9..words.len() {
            let pure_t: &str;
            if words[t].ends_with(",") {
                let (split_t, _) = words[t].split_once(",").unwrap();
                pure_t = split_t;
            } else {
                pure_t = words[t];
            }
            tunnels.push(pure_t);
        }
        // println!("room:{:?}", room);
        // println!("flow:{:?}", flow_rate);
        // println!("tunnels:{:?}", tunnels);
        rooms.insert(room, (flow_rate, tunnels));
    }

    rooms
}

fn main() {
    let t = Instant::now();
    let input = fs::read_to_string("./input/16.txt").unwrap();
    println!("p1: {}", p1(&input));
    println!("Ran p1 in {:.4?}", t.elapsed());
    let t = Instant::now();
    println!("p2: {}", p2(&input));
    println!("Ran p2 in {:.4?}", t.elapsed());
}

fn p1(input: &str) -> String {
    let mut current_room = "AA";
    let max_mins = 30;
    let rooms = parse(input);

    for (room, (flow_rate, tunnels)) in rooms.iter() {
        println!("{:?} - {:?} - {:?}", room, flow_rate, tunnels);
        let mut t_flows: i32 = tunnels
            .iter()
            .map(|t| {
                let (f, _) = rooms.get(t).unwrap();
                *f
            })
            .sum();
        let total_flow: i32 = t_flows + flow_rate;
        println!("total flow: {:?}", total_flow);
    }

    println!("{:?}", rooms);

    "foo".to_string()
}

fn p2(input: &str) -> String {
    "bar".to_string()
}

#[cfg(test)]
mod d16 {
    use super::*;

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn t1() {
        let result = p1(INPUT);
        assert_eq!(result, "1651");
    }

    #[test]
    fn t2() {
        let result = p2(INPUT);
        assert_eq!(result, "bar");
    }
}
