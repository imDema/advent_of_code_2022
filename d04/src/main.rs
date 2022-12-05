use std::ops::RangeInclusive;
use std::str::FromStr;

use eyre::Result;
use itertools::Itertools;

struct Pair {
    a: RangeInclusive<usize>,
    b: RangeInclusive<usize>,
}

impl FromStr for Pair {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split(',').collect_tuple().unwrap();

        let (a1, a2) = a
            .split('-')
            .map(|q| q.parse().unwrap())
            .collect_tuple()
            .unwrap();
        let (b1, b2) = b
            .split('-')
            .map(|q| q.parse().unwrap())
            .collect_tuple()
            .unwrap();

        Ok(Self {
            a: a1..=a2,
            b: b1..=b2,
        })
    }
}

fn check(Pair { a, b }: &Pair) -> bool {
    a.start() <= b.start() && a.end() >= b.end() || b.start() <= a.start() && b.end() >= a.end()
}

fn check_2(Pair { a, b }: &Pair) -> bool {
    a.end() >= b.start() && a.start() <= b.end() || b.end() >= a.start() && b.start() <= a.end()
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin())?;

    let r = input
        .lines()
        .map(|l| l.parse().unwrap())
        .filter(check)
        .count();

    println!("{r}");

    let r = input
        .lines()
        .map(|l| l.parse().unwrap())
        .filter(check_2)
        .count();

    println!("{r}");

    Ok(())
}
