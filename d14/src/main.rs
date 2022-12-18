#![allow(unused_imports)]

use std::collections::HashMap;

use eyre::Result;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::bytes::*;
use nom::character::complete::one_of;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::separated_pair;
use nom::Finish;
use nom::IResult;

const SOURCE: Point = (500, 0);

// Input format:
// 498,4 -> 498,6 -> 496,6
// 503,4 -> 502,4 -> 502,9 -> 494,9

type Point = (isize, isize);

fn decimal(s: &str) -> IResult<&str, isize> {
    let (s, r) = recognize(many1(one_of("0123456789")))(s)?;
    Ok((s, r.parse().unwrap()))
}

fn point(s: &str) -> IResult<&str, Point> {
    separated_pair(decimal, tag(","), decimal)(s)
}

fn path(s: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(tag(" -> "), point)(s)
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum Tile {
    #[default]
    None,
    Block,
    Sand,
}

#[derive(Default)]
struct Map {
    grid: HashMap<Point, Tile>,
    floor: isize,
}

impl Map {
    fn add_segment(&mut self, a: Point, b: Point) {
        match (a.0 == b.0, a.1 == b.1) {
            (true, true) => {
                self.grid.insert(a, Tile::Block);
            }
            (true, false) => {
                for j in (a.1.min(b.1))..=(a.1.max(b.1)) {
                    self.grid.insert((a.0, j), Tile::Block);
                }
            }
            (false, true) => {
                for i in (a.0.min(b.0))..=(a.0.max(b.0)) {
                    self.grid.insert((i, a.1), Tile::Block);
                }
            }
            (false, false) => panic!("invalid path"),
        }
        self.floor = self.floor.max(a.1).max(b.1);
    }

    fn sim_step(&mut self, p: Point) -> bool {
        if !matches!(self.grid.entry(p).or_default(), Tile::None) {
            eprintln!("point is blocked!");
            return false;
        }

        if p.1 == self.floor {
            return true;
        }

        macro_rules! try_side {
            ($e:expr) => {
                if let Tile::None = self.grid.entry($e).or_default() {
                    return self.sim_step($e);
                }
            };
        }

        try_side!((p.0, p.1 + 1));
        try_side!((p.0 - 1, p.1 + 1));
        try_side!((p.0 + 1, p.1 + 1));

        assert!(matches!(
            self.grid[&(p.0, p.1 + 1)],
            Tile::Block | Tile::Sand
        ));

        self.grid.insert(p, Tile::Sand);
        return false;
    }

    fn sim_step_2(&mut self, p: Point) -> bool {
        if !matches!(self.grid.entry(p).or_default(), Tile::None) {
            eprintln!("point is blocked!");
            return true;
        }

        if p.1 + 1 == self.floor {
            self.grid.insert(p, Tile::Sand);
            return false;
        }

        macro_rules! try_side {
            ($e:expr) => {
                if let Tile::None = self.grid.entry($e).or_default() {
                    return self.sim_step_2($e);
                }
            };
        }

        try_side!((p.0, p.1 + 1));
        try_side!((p.0 - 1, p.1 + 1));
        try_side!((p.0 + 1, p.1 + 1));

        assert!(matches!(
            self.grid[&(p.0, p.1 + 1)],
            Tile::Block | Tile::Sand
        ));

        self.grid.insert(p, Tile::Sand);
        return false;
    }
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin())?;

    // PART 1
    let paths: Vec<_> = input.lines().map(|l| path(l).finish().unwrap().1).collect();
    let mut map = Map::default();
    paths
        .iter()
        .for_each(|p| p.windows(2).for_each(|w| map.add_segment(w[0], w[1])));

    while !map.sim_step(SOURCE) {}

    let r = map.grid.values().filter(|&q| *q == Tile::Sand).count();
    println!("{r}");

    // PART 2
    map.floor += 2;

    while !map.sim_step_2(SOURCE) {}

    let r = map.grid.values().filter(|&q| *q == Tile::Sand).count();
    println!("{r}");

    // println!("{r}");
    Ok(())
}
