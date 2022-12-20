use std::{collections::HashSet, fs, time::Instant};

fn parse(input: &str, y: i32) -> (Vec<((i32, i32), (i32, i32))>, HashSet<(i32, i32)>, i32) {
    let mut beacons_on_y = 0;
    let mut beacons: HashSet<(i32, i32)> = HashSet::new();
    let sensors: Vec<((i32, i32), (i32, i32))> = input
        .split("\n")
        .map(|line| {
            println!("line: {:?}", line);
            let (_, sx) = line.split_at(12);
            let (sx, sy) = sx.split_once(", y=").unwrap();
            let (sy, bx) = sy.split_once(": closest beacon is at x=").unwrap();
            let (bx, by) = bx.split_once(", y=").unwrap();

            let sensor: (i32, i32) = (sx.parse().unwrap(), sy.parse().unwrap());
            let beacon: (i32, i32) = (bx.parse().unwrap(), by.parse().unwrap());
            if beacon.1 == y && !beacons.contains(&beacon) {
                beacons_on_y += 1;
            }

            beacons.insert(beacon);
            (sensor, beacon)
        })
        .collect();

    (sensors, beacons, beacons_on_y)
}

fn main() {
    let t = Instant::now();
    let input = fs::read_to_string("./input/15.txt").unwrap();
    println!("p1: {}", p1(&input, 2_000_000));
    println!("Ran p1 in {:.4?}", t.elapsed());
    let t = Instant::now();
    println!("p2: {}", p2(&input, 4_000_000));
    println!("Ran p2 in {:.4?}", t.elapsed());
}

fn p1(input: &str, y: i32) -> String {
    let (sensors, beacons, beacons_on_y) = parse(input, y);

    let mut blocked_y: HashSet<(i32, i32)> = HashSet::new();

    let mut blocked_ranges: Vec<(i32, i32)> = vec![];
    let mut min_x_blocked: Option<i32> = None;
    let mut max_x_blocked: Option<i32> = None;

    for i in 0..sensors.len() {
        let (s, b) = sensors[i];
        let dx = (s.0 - b.0).abs();
        let dy = (s.1 - b.1).abs();
        let dist = dx.abs() + dy.abs();

        if s.1 - dist <= y && s.1 + dist >= y {
            println!("{:?} - OVERLAPS ON Y", i);
            println!("sensors {:?}: {:?}", i, sensors[i]);
            println!("dx, dy: {:?},{:?}", dx, dy);
            println!("dx + dy: {:?}", dist);
            println!(
                "x range: {:?},{:?} - {:?},{:?}",
                s.0 - dist,
                s.1,
                s.0 + dist,
                s.1
            );

            println!(
                "y range: {:?},{:?} - {:?},{:?}",
                s.0,
                s.1 - dist,
                s.0,
                s.1 + dist
            );
            let overlapping_y = (dist * 2 + 1) - ((s.1 - y).abs() * 2);
            let overlapping_dy = overlapping_y / 2;

            println!("overlapping_y: {:?} - {:?}", overlapping_y, overlapping_dy);

            println!(
                "range on y: {:?},{:?} - {:?},{:?}",
                s.0 - overlapping_dy,
                y,
                s.0 + overlapping_dy,
                y
            );

            let min_x = s.0 - overlapping_dy;
            let max_x = s.0 + overlapping_dy;

            blocked_ranges.push((min_x, max_x));

            match min_x_blocked {
                None => min_x_blocked = Some(min_x),
                Some(_) => {
                    if min_x < min_x_blocked.unwrap() {
                        min_x_blocked = Some(min_x);
                    }
                }
            };

            match max_x_blocked {
                None => max_x_blocked = Some(max_x),
                Some(_) => {
                    if max_x > max_x_blocked.unwrap() {
                        max_x_blocked = Some(max_x);
                    }
                }
            };

            // for n in 0..overlapping_y {
            //     let overlapping_point = ((s.0 - overlapping_dy) + n, y);

            //     blocked_y.insert(overlapping_point);
            //     // println!("n: {:?} - {:?}", n, overlapping_point);
            // }
        }
    }

    println!("y: {:?}", y);

    println!("beacons_on_y: {:?}", beacons_on_y);
    // println!("sensors: {:?}", sensors);
    println!("beacons: {:?}", beacons);
    for i in 0..blocked_ranges.len() {
        let (min, max) = blocked_ranges[i];
        println!("blocked_ranges {:?}: {:?} - {:?}", i, min, max);
    }

    let min_x = min_x_blocked.unwrap();
    let max_x = max_x_blocked.unwrap();

    let mut total = 0;
    for _i in min_x..=max_x {
        // println!("- total: {:?}", total);
        total += 1;
    }
    total -= beacons_on_y;
    println!("total: {:?}", total);

    total.to_string()
}

fn p2(input: &str, max: i32) -> String {
    let mut beacon_pos: (i32, i32) = (0, 0);
    let tuning_freq = max * beacon_pos.0 + beacon_pos.1;
    tuning_freq.to_string()
}

#[cfg(test)]
mod d15 {
    use super::*;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn t1() {
        let result = p1(INPUT, 10);
        assert_eq!(result, "26");
    }

    #[test]
    fn t2() {
        let result = p2(INPUT, 20);
        assert_eq!(result, "56000011");
    }
}
