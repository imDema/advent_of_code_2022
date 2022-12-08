use std::{collections::HashMap, iter::Peekable};

use eyre::Result;
use itertools::Itertools;

#[derive(Default)]
struct Node {
    size: usize,
    children: HashMap<String, Node>,
}

impl Node {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn parse<'a, I: Iterator<Item = String>>(&mut self, lines: &'a mut Peekable<I>) {
        while let Some(cmd) = lines.next() {
            match &cmd[..4] {
                "$ ls" if self.size == 0 => {
                    while !lines.peek().map(|l| l.starts_with('$')).unwrap_or(true) {
                        let file = lines.next().unwrap();
                        let (size, name) = file.split_whitespace().collect_tuple().unwrap();
                        match size {
                            "dir" => {
                                self.children.insert(name.to_string(), Node::new());
                            }
                            s => self.size += s.parse::<usize>().unwrap(),
                        }
                    }
                }
                "$ cd" => {
                    let dest = cmd.split_whitespace().nth(2).unwrap();
                    eprintln!("{dest}");
                    match dest {
                        ".." => return,
                        "/" => continue,
                        d => self.children.get_mut(&d.to_string()).expect("cd to unlisted dir").parse(lines),
                    }
                }
                "$ ls" if self.size != 0 => eprintln!("ignoring duplicate ls"),
                _ => panic!("invalid command {cmd}"),
            }
        }
    }

    pub fn part_1(&self) -> (usize, usize) {
        let (sum, score) = self.children.iter()
            .map(|(_, v)| v.part_1())
            .fold((self.size, 0), |(sum, score), (sub_sum, sub_score)| (sum + sub_sum, score + sub_score));

        if sum < 100000 {
            (sum, score + sum)
        } else {
            (sum, score)
        }
    }

    pub fn part_2(&self, min_size: usize) -> (usize, usize) {
        let (sum, min_score) = self.children.iter()
            .map(|(_, v)| v.part_2(min_size))
            .fold((self.size, usize::MAX), |(sum, score), (sub_sum, sub_score)| (sum + sub_sum, score.min(sub_score)));

        if sum > min_size {
            (sum, sum.min(min_score))
        } else {
            (sum, min_score)
        }
    }
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin())?;
    
    // PART 1
    let mut lines = input.lines().map(String::from).peekable();
    let mut tree = Node::new();
    
    tree.parse(&mut lines);
    let (sum, score) = tree.part_1();
    println!("{score}");

    // PART 2
    let delta = sum - 40000000;
    let (_, score) = tree.part_2(delta);
    println!("{score}");
    
    Ok(())
}
