#![allow(unused_imports)]

use eyre::Result;
use itertools::Itertools;
use regex::Regex;
use once_cell::sync::Lazy;

static RE_MONKE: Lazy<Regex> = Lazy::new(|| Regex::new(
    r#"\s*Monkey \d+:
\s*Starting items: (?P<items>.+)
\s*Operation: new = (?P<op>.+)
\s*Test: divisible by (?P<div>\d+)
\s*If true: throw to monkey (?P<dest_true>\d+)
\s*If false: throw to monkey (?P<dest_false>\d+)"#
).unwrap());
static RE_OP: Lazy<Regex> = Lazy::new(|| Regex::new(r"old (\*|\+) (\w+)").unwrap());
static RE_NUMBER: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

#[derive(Debug)]
enum Op {
    Sq,
    Add(usize),
    Mul(usize)
}

impl Op {
    pub fn parse(s: &str) -> Self {
        let caps = RE_OP.captures(s).unwrap();

        match (&caps[1], &caps[2]) {
            ("*", "old") => Self::Sq,
            ("*", d) => Self::Mul(d.parse().unwrap()),
            ("+", d) => Self::Add(d.parse().unwrap()),
            _ => panic!("invalid op"),
        }
    }

    pub fn apply(&self, rhs: usize) -> usize {
        match self {
            Op::Sq => rhs * rhs,
            Op::Add(a) => rhs + a,
            Op::Mul(a) => rhs * a,
        }
    }
}

struct Message {
    dest: usize,
    value: usize,
}

#[derive(Debug)]
struct Monke {
    items: Vec<usize>,
    op: Op,
    divisor: usize,
    dest_true: usize,
    dest_false: usize,
    inspect_count: usize,
}

impl Monke {
    pub fn parse(s: &str) -> Self {
        let caps = RE_MONKE.captures(s).expect("Wrong input format");

        let items = caps.name("items").unwrap().as_str();
        let items = RE_NUMBER.captures_iter(items).map(|d| d[0].parse().unwrap()).collect();

        let op = caps.name("op").unwrap().as_str();
        let op = Op::parse(op);

        let div = caps.name("div").unwrap().as_str().parse().unwrap();
        let dest_true = caps.name("dest_true").unwrap().as_str().parse().unwrap();
        let dest_false = caps.name("dest_false").unwrap().as_str().parse().unwrap();

        Self {
            items,
            op,
            divisor: div,
            dest_true,
            dest_false,
            inspect_count: 0,
        }
    }

    pub fn inspect_all(&mut self) -> impl IntoIterator<Item=Message> {
        self.inspect_count += self.items.len();
        self.items.drain(..)
            .map(|o| self.op.apply(o) / 3)
            .map(|value| if value % self.divisor == 0 {
                Message{ dest: self.dest_true, value}
            } else {
                Message{ dest: self.dest_false, value}
            })
            .collect::<Vec<_>>()
    }

    pub fn inspect_all_2(&mut self, modulo: usize) -> impl IntoIterator<Item=Message> {
        self.inspect_count += self.items.len();
        self.items.drain(..)
            .map(|o| self.op.apply(o) % modulo)
            .map(|value| if value % self.divisor == 0 {
                Message{ dest: self.dest_true, value}
            } else {
                Message{ dest: self.dest_false, value}
            })
            .collect::<Vec<_>>()
    }
}


fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin())?;

    // PART 1
    let mut monkeys = input.split("\n\n")
        .map(Monke::parse)
        .collect::<Vec<_>>();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for message in monkeys[i].inspect_all() {
                monkeys[message.dest].items.push(message.value);
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspect_count);

    eprintln!("{monkeys:?}");

    println!("{}", monkeys[monkeys.len()-1].inspect_count * monkeys[monkeys.len()-2].inspect_count);

    // PART 2
    let mut monkeys = input.split("\n\n")
        .map(Monke::parse)
        .collect::<Vec<_>>();

    let modulo = monkeys.iter().map(|m| m.divisor).product::<usize>();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            for message in monkeys[i].inspect_all_2(modulo) {
                monkeys[message.dest].items.push(message.value);
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspect_count);

    eprintln!("{monkeys:#?}");

    println!("{}", monkeys[monkeys.len()-1].inspect_count * monkeys[monkeys.len()-2].inspect_count);

    // println!("{r}");
    Ok(())
}
