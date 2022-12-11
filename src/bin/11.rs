use std::{fs, time::Instant};

type Monkey = (Vec<usize>, String, usize, usize, usize);

fn get_monkeys(input: &str) -> Vec<Monkey> {
    let mut monkeys = vec![];

    let groups = input.split("\n\n").map(|g| g).collect::<Vec<_>>();
    for group in groups {
        let monkey = group.split("\n").map(|g| g).collect::<Vec<_>>();
        let items = monkey[1]
            .split_at(18)
            .1
            .split(", ")
            .map(|i| i.parse::<usize>().expect("Found non-int item"))
            .collect::<Vec<_>>();

        let op = monkey[2].split_at(23).1.to_string();
        let divisor = monkey[3]
            .split_at(21)
            .1
            .parse::<usize>()
            .expect("Non int divisor found");
        let t1 = monkey[4]
            .split_once("monkey ")
            .unwrap()
            .1
            .parse::<usize>()
            .expect("Non int t1 found");
        let t2 = monkey[5]
            .split_once("monkey ")
            .unwrap()
            .1
            .parse::<usize>()
            .expect("Non int t2 found");

        // println!("t1: {:?} - t2: {:?}", t1, t2);
        monkeys.push((items, op, divisor, t1, t2))
    }

    monkeys
}

fn do_op(worry: usize, op: String) -> usize {
    let (operand, raw_operator) = op.split_once(" ").unwrap();
    let operator = match raw_operator {
        "old" => worry,
        _ => raw_operator.parse::<usize>().unwrap(),
    };
    match operand {
        "+" => worry + operator,
        "*" => worry * operator,
        _ => panic!("Unknown op handled: {:?}", op),
    }
}

fn main() {
    let t = Instant::now();
    let input = fs::read_to_string("./input/11.txt").unwrap();
    println!("p1: {}", p1(&input));
    println!("Ran p1 in {:.4?}", t.elapsed());
    let t = Instant::now();
    println!("p2: {}", p2(&input));
    println!("Ran p2 in {:.4?}", t.elapsed());
}

fn p1(input: &str) -> String {
    let mut monkeys = get_monkeys(input);
    let mut inspections = vec![0; monkeys.len()];
    let rounds = 20;
    for _round in 0..rounds {
        for i in 0..monkeys.len() {
            for j in 0..monkeys[i].0.len() {
                let item = monkeys[i].0[j];
                let mut worry = do_op(item, monkeys[i].1.clone());
                worry = worry / 3;

                let target = if worry % monkeys[i].2 == 0 {
                    monkeys[i].3
                } else {
                    monkeys[i].4
                };
                monkeys[target].0.push(worry);
            }
            inspections[i] += monkeys[i].0.len();
            monkeys[i].0 = vec![];
        }
    }

    inspections.sort();
    (inspections.pop().unwrap() * inspections.pop().unwrap()).to_string()
}

fn p2(input: &str) -> String {
    let mut monkeys = get_monkeys(input);
    let mut inspections = vec![0; monkeys.len()];
    let mut worry_mod = 1;
    for m in 0..monkeys.len() {
        worry_mod *= monkeys[m].2;
    }

    let rounds = 10_000;
    for _round in 0..rounds {
        for i in 0..monkeys.len() {
            for j in 0..monkeys[i].0.len() {
                let item = monkeys[i].0[j];
                let mut worry = do_op(item, monkeys[i].1.clone());
                worry %= worry_mod;

                let target = if worry % monkeys[i].2 == 0 {
                    monkeys[i].3
                } else {
                    monkeys[i].4
                };
                monkeys[target].0.push(worry);
            }
            inspections[i] += monkeys[i].0.len();
            monkeys[i].0 = vec![];
        }
    }
    inspections.sort();
    (inspections.pop().unwrap() * inspections.pop().unwrap()).to_string()
}

#[cfg(test)]
mod d11 {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn t1() {
        let result = p1(INPUT);
        assert_eq!(result, "10605");
    }

    #[test]
    fn t2() {
        let result = p2(INPUT);
        assert_eq!(result, "2713310158");
    }
}
