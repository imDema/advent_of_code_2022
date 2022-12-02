use std::io::stdin;

use eyre::Result;
use itertools::Itertools;

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

fn parse_line(l: &str) -> (u8, u8) {
    let (a, b) = l.split(' ').collect_tuple().unwrap();
    (
        a.chars().next().unwrap() as u8 - b'A',
        b.chars().next().unwrap() as u8 - b'X',
    )
}

fn diff(a: u8, b: u8) -> u8 {
    (a + 3 - b) % 3
}

fn result(a: u8, b: u8) -> Outcome {
    match diff(a, b) {
        0 => Outcome::Draw,
        1 => Outcome::Lose,
        2 => Outcome::Win,
        _ => unreachable!("invalid play"),
    }
}

fn shape_score(b: u8) -> usize {
    match b {
        0 => 1,
        1 => 2,
        2 => 3,
        _ => unreachable!("invalid play"),
    }
}

fn score((a, b): (u8, u8)) -> usize {
    result(a, b).score() + shape_score(b)
}

fn score_2((a, res): (u8, u8)) -> usize {
    let delta = match res {
        0 => 2,
        1 => 0,
        2 => 1,
        _ => unreachable!("invalid play"),
    };
    let b = (a + delta) % 3;
    result(a, b).score() + shape_score(b)
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(stdin())?;

    let r: usize = input.lines()
        .map(parse_line)
        .map(score)
        .sum();

    println!("{r}");

    let r: usize = input.lines()
        .map(parse_line)
        .map(score_2)
        .sum();

    println!("{r}");

    Ok(())
}
