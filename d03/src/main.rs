use std::collections::HashSet;
use std::io::stdin;

use eyre::Result;
use itertools::Itertools;

fn char_prio(c: char) -> u8 {
    match c {
        'a'..='z' => c as u8 - b'a' + 1,
        'A'..='Z' => c as u8 - b'A' + 27,
        _ => unreachable!(),
    }
}

fn part_1(s: &str) -> usize {
    let s1: HashSet<u8> = s.chars().map(char_prio).take(s.len() / 2).collect();
    let s2: HashSet<u8> = s.chars().map(char_prio).skip(s.len() / 2).collect();

    *s1.intersection(&s2).next().unwrap() as usize
}

fn part_2<'a>(s: impl IntoIterator<Item = &'a str>) -> usize {
    let mut sets: Vec<HashSet<_>> = s
        .into_iter()
        .map(|l| l.chars().map(char_prio).collect())
        .collect();

    let int = sets.pop().unwrap();
    sets.into_iter()
        .fold(int, |acc, set| acc.intersection(&set).cloned().collect())
        .into_iter()
        .map(usize::from)
        .sum()
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(stdin())?;

    let r: usize = input.lines().map(part_1).sum();

    println!("{r}");

    let r: usize = input.lines().chunks(3).into_iter().map(part_2).sum();

    println!("{r}");

    Ok(())
}
