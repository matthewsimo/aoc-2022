use std::{cell::RefCell, collections::HashMap, fs, rc::Rc};

#[derive(Default)]
struct Dir {
    _name: String,
    size: RefCell<usize>,
    parent: Option<Rc<Dir>>,
    contents: RefCell<HashMap<String, Rc<Dir>>>,
}

impl Dir {
    fn get_size(&self) -> usize {
        *self.size.borrow()
            + self
                .contents
                .borrow()
                .values()
                .fold(0, |a, b| a + b.get_size())
    }
}

fn main() {
    let input = fs::read_to_string("./input/07.txt").unwrap();
    println!("p1: {}", p1(&input));
    println!("p2: {}", p2(&input));
}

fn get_filesystem(input: &str) -> Rc<Dir> {
    let root = Rc::new(Dir::default());

    let mut current_dir = Rc::clone(&root);
    let lines: Vec<String> = aoc::parse_lines(String::from(input));

    for line in lines {
        let words = line.split(' ').collect::<Vec<&str>>();
        match (words[0], words[1]) {
            ("$", "ls") => {}
            ("$", "cd") => match words[2] {
                "/" => current_dir = Rc::clone(&root),
                ".." => current_dir = Rc::clone(&current_dir.parent.as_ref().unwrap()),
                dirname => {
                    let newdir = current_dir.contents.borrow().get(dirname).unwrap().clone();
                    current_dir = newdir;
                }
            },
            ("dir", dirname) => {
                current_dir.contents.borrow_mut().insert(
                    dirname.to_string(),
                    Rc::new(Dir {
                        _name: dirname.to_string(),
                        size: RefCell::new(0),
                        parent: Some(Rc::clone(&current_dir)),
                        contents: RefCell::new(HashMap::new()),
                    }),
                );
            }
            (size, _name) => {
                *current_dir.size.borrow_mut() += size.parse::<usize>().unwrap();
            }
        }
    }

    root
}

fn p1(input: &str) -> String {
    let root = get_filesystem(input);
    let mut to_visit = vec![Rc::clone(&root)];
    let mut total = 0;

    while let Some(dir) = to_visit.pop() {
        to_visit.extend(dir.contents.borrow().values().map(Rc::clone));

        let size = dir.get_size();
        if size <= 100000 {
            total += size;
        }
    }

    total.to_string()
}

fn p2(input: &str) -> String {
    let root = get_filesystem(input);
    let total_size = root.get_size();
    let free_space = 70000000 - total_size;
    let space_needed = 30000000 - free_space;

    let mut to_visit = vec![Rc::clone(&root)];
    let mut best = usize::MAX;

    while let Some(dir) = to_visit.pop() {
        to_visit.extend(dir.contents.borrow().values().map(Rc::clone));

        let size = dir.get_size();
        if size >= space_needed {
            best = best.min(size);
        }
    }

    best.to_string()
}

#[cfg(test)]
mod d07 {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn t1() {
        let result = p1(INPUT);
        assert_eq!(result, "95437");
    }

    #[test]
    fn t2() {
        let result = p2(INPUT);
        assert_eq!(result, "24933642");
    }
}
