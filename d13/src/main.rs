#![allow(unused_imports)]

use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::str::FromStr;

use eyre::Result;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::combinator::*;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;

///! Note that likely a better implementation would have been to parse the packet as a list of (value, depth)

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tok {
    Enter,
    Exit,
    Value(u32),
}

impl Debug for Tok {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Tok {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tok::Enter => write!(f, "["),
            Tok::Exit => write!(f, "]"),
            Tok::Value(v) => write!(f, "{v}"),
        }
    }
}

impl FromStr for Tok {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "[" => Self::Enter,
            "]" => Self::Exit,
            a => Self::Value(u32::from_str_radix(a, 10).map_err(|_| ())?),
        })
    }
}

fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(one_of("0123456789")))(input)
}

fn parse_packet(s: &str) -> IResult<&str, Vec<Tok>> {
    many1(map_res(
        terminated(alt((recognize(one_of("[]")), decimal)), opt(tag(","))),
        Tok::from_str,
    ))(s)
}

#[derive(Debug)]
struct Cruncher {
    left: Vec<Tok>,
    right: Vec<Tok>,
    l: usize,
    r: usize,
}

impl Cruncher {
    fn new(left: Vec<Tok>, right: Vec<Tok>) -> Self {
        Self {
            left,
            right,
            l: 0,
            r: 0,
        }
    }

    fn parse(s: &str) -> Self {
        let (left, right) = s
            .lines()
            .map(|l| parse_packet(l).unwrap().1)
            .collect_tuple()
            .unwrap();
        Self::new(left, right)
    }

    fn cur(&self) -> (Tok, Tok) {
        (self.left[self.l], self.right[self.r])
    }

    #[inline(always)]
    fn step_compare(&mut self) -> Ordering {
        self.l += 1;
        self.r += 1;
        return self.compare();
    }

    fn compare(&mut self) -> Ordering {
        if self.l == self.left.len() && self.r == self.right.len() {
            return Ordering::Equal;
        }
        if self.r == self.right.len() {
            return Ordering::Greater;
        }
        match self.cur() {
            (Tok::Value(l), Tok::Value(r)) => match l.cmp(&r) {
                std::cmp::Ordering::Less => return Ordering::Less,
                std::cmp::Ordering::Greater => return Ordering::Greater,
                std::cmp::Ordering::Equal => return self.step_compare(),
            },
            (Tok::Value(_) | Tok::Enter, Tok::Exit) => return Ordering::Greater,
            (Tok::Exit, Tok::Value(_) | Tok::Enter) => return Ordering::Less,
            (Tok::Exit, Tok::Exit) => return self.step_compare(),
            (Tok::Enter, Tok::Enter) => return self.step_compare(),
            (Tok::Enter, Tok::Value(_)) => {
                self.right.insert(self.r, Tok::Enter);
                self.right.insert(self.r + 2, Tok::Exit);
                return self.compare();
            }
            (Tok::Value(_), Tok::Enter) => {
                self.left.insert(self.l, Tok::Enter);
                self.left.insert(self.l + 2, Tok::Exit);
                return self.compare();
            }
        }
    }
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin())?;

    // PART 1
    let r = input
        .split("\n\n")
        .map(Cruncher::parse)
        .enumerate()
        .filter_map(|(i, mut c)| {
            if c.compare().is_le() {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum::<usize>();

    println!("{r}");

    // PART 2
    let mut v = input
        .lines()
        .filter(|l| l.len() > 0)
        .chain(["[[2]]", "[[6]]"])
        .map(|l| parse_packet(l).unwrap().1)
        .collect::<Vec<_>>();

    v.sort_by(|a, b| {
        let mut c = Cruncher::new(a.clone(), b.clone()); // Inefficient cloning!
        c.compare()
    });

    let m1 = parse_packet("[[2]]").unwrap().1;
    let m2 = parse_packet("[[6]]").unwrap().1;
    let a = v.iter().find_position(move |&q| *q == m1).unwrap().0 + 1;
    let b = v.iter().find_position(move |&q| *q == m2).unwrap().0 + 1;

    let r = a * b;

    println!("{r}");
    Ok(())
}
