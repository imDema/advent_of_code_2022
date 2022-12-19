#![allow(unused_imports)]

use std::ops::Range;

use eyre::Result;
use fxhash::FxHashMap;
use fxhash::FxHashSet;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::separated_pair;
use nom::Finish;
use nom::IResult;

// Input format:
// Sensor at x=2, y=18: closest beacon is at x=-2, y=15

type Point = (i64, i64);

fn decimal(s: &str) -> IResult<&str, i64> {
    let (s, r) = recognize(many1(one_of("-0123456789")))(s)?;
    Ok((s, r.parse().unwrap()))
}

fn point(s: &str) -> IResult<&str, Point> {
    separated_pair(decimal, tag(", y="), decimal)(s)
}

fn reading(s: &str) -> IResult<&str, (Point, Point)> {
    let (s, _) = tag("Sensor at x=")(s)?;
    let (s, p1) = point(s)?;
    let (s, _) = tag(": closest beacon is at x=")(s)?;
    let (s, p2) = point(s)?;
    Ok((s, (p1, p2)))
}

fn distance(a: Point, b: Point) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

#[derive(Default)]
struct Map {
    sensors: FxHashMap<Point, i64>,
    beacons: FxHashSet<Point>,
    bound_x: Range<i64>,
    bound_y: Range<i64>,
}

impl Map {
    fn mark_sensor(&mut self, s: Point, b: Point) {
        let d = distance(s, b);
        self.sensors.insert(s, d);
        self.beacons.insert(b);
        self.bound_x.start = self.bound_x.start.min(s.0 - d);
        self.bound_x.end = self.bound_x.start.max(s.0 + d);
        self.bound_y.start = self.bound_y.start.min(s.1 - d);
        self.bound_y.end = self.bound_y.start.max(s.1 + d);
    }

    fn check_beacon(&self, p: Point) -> bool {
        self.beacons.contains(&p)
    }

    fn check_blocked(&self, p: Point) -> bool {
        self.sensors.iter().any(|(s, d)| distance(*s, p) <= *d)
    }
}

struct ManhattanCircleIter {
    c: Point,
    d: i64,
    idx: i64,
}

impl Iterator for ManhattanCircleIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.idx % self.d;
        let next = match self.idx / self.d {
            0 => (self.c.0 - self.d + i, self.c.1 + i),
            1 => (self.c.0 + i, self.c.1 + self.d - i),
            2 => (self.c.0 + self.d - i, self.c.1 - i),
            3 => (self.c.0 - i, self.c.1 - self.d + i),
            4.. => return None,
            _ => unreachable!(),
        };
        self.idx += 1;
        Some(next)
    }
}

fn manhattan_circle(c: Point, d: i64) -> impl Iterator<Item=Point> {
    ManhattanCircleIter {
        c,
        d,
        idx: 0,
    }
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin())?;

    // PART 1
    let mut map = Map::default();

    input.lines().map(|l| reading(l).finish().unwrap().1)
        .for_each(|(s, b)| map.mark_sensor(s, b));
    
    let r = map.bound_x.clone().filter(|i| !map.check_beacon((*i, 2000000)) && map.check_blocked((*i, 2000000))).count();
    
    println!("{r}");

    let p = map.sensors.iter()
        .flat_map(|(s, d)| manhattan_circle(*s, *d + 1))
        .filter(|p| (0..4000000).contains(&p.0) && (0..4000000).contains(&p.1))
        .find(|&p| !map.check_blocked(p))
        .unwrap();

    let (i, j) = p;
    println!("{}", i.checked_mul(4000000).unwrap().checked_add(j).unwrap());
    Ok(())
}
