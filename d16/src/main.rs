#![allow(unused_imports)]

///! I'm not really proud of this solution :(

use std::{collections::{HashMap, HashSet}, mem::swap, borrow::Cow};

use eyre::Result;
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_till, take_while},
    character::complete::{alpha1, digit1},
    combinator::{recognize, opt},
    multi::{many1, separated_list1},
    IResult,
};
use petgraph::{prelude::*, dot::{Dot, Config}, algo::floyd_warshall};
use petgraph::algo::dijkstra;

// Valve KR has flow rate=17; tunnels lead to valves WA, JQ, JY, KI

fn parse_valve(s: &str) -> IResult<&str, (String, Node)> {
    let (s, _) = tag("Valve ")(s)?;
    let (s, name) = alpha1(s)?;
    let (s, _) = take_till(|c: char| c.is_ascii_digit())(s)?;
    let (s, k) = digit1(s)?;
    let (s, _) = take_till(|c: char| c.is_uppercase())(s)?;
    let (s, succ) = separated_list1(tag(", "), alpha1)(s)?;

    Ok((
        s,
        (
            name.to_string(),
            Node {
                k: k.parse().unwrap(),
                succ: succ.into_iter().map(String::from).collect(),
            },
        ),
    ))
}

struct Node {
    k: usize,
    succ: Vec<String>,
}

struct SearchGraph<'a> {
    values: HashMap<&'a str, isize>,
    metagraph: HashMap<(&'a str, &'a str), isize>,
}

#[derive(Clone, Copy)]
struct SearchState<'a> {
    a: &'a str,
    b: &'a str,
    time_a: isize,
    time_b: isize,
}

impl<'a> From<&'a HashMap<String, Node>> for SearchGraph<'a> {
    fn from(value: &'a HashMap<String, Node>) -> Self {
        let mut graph: GraphMap<&str, isize, Directed> = GraphMap::new();

        for (a, v) in value.iter() {
            for b in v.succ.iter() {
                graph.add_edge(a.as_str(), b.as_str(), 1);
            }
        }
        let st = floyd_warshall(&graph, |e| *e.2).unwrap();

        let values = value.iter()
            .map(|(k, v)| (k.as_str(), v.k as isize))
            .filter(|(_, v)| *v > 0)
            .collect();

        Self { metagraph: st, values  }
    }
}

impl<'a> SearchGraph<'a> {
    fn compute_score(&self, activated: &HashMap<&'a str, isize>) -> isize {
        activated.into_iter()
            .map(|(n, t)| self.values[n] * t)
            .sum()
    }

    fn search(&self, start: &str, time: isize, activated: &mut HashMap<&'a str, isize>) -> isize {
        let mut max = 0;
        for (&name, _) in &self.values {
            if activated.contains_key(name) {
                continue;
            }
            let d = self.metagraph[&(start, name)] + 1;
            if time <= d {
                continue;
            }
            activated.insert(name, time - d);
            max = max.max(self.search(name, time - d, activated));
            activated.remove(name);
        }

        max.max(self.compute_score(activated))
    }

    fn neighbours(&'a self, s: SearchState<'a>, avail: &HashSet<&'a str>) -> impl Iterator<Item=SearchState<'a>> {
        let moves = avail.iter()
            .map(move |&n| (n, self.metagraph[&(s.a, n)] + 1))
            .map(move |(n, cost)| SearchState {
                a: n,
                b: s.b,
                time_a: s.time_a - cost,
                time_b: s.time_b,
            })
            .filter(|ns| ns.a != s.a && ns.time_a >= 0)
            .collect::<Vec<_>>();

        if !moves.is_empty() {
            return moves.into_iter();
        }

        avail.iter()
            .map(move |&n| (n, self.metagraph[&(s.b, n)] + 1))
            .map(move |(n, cost)| SearchState {
                a: s.a,
                b: n,
                time_a: s.time_a,
                time_b: s.time_b - cost,
            })
            .filter(|ns| ns.b != s.b && ns.time_b >= 0)
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn search_2(&'a self, state: &SearchState<'a>, activated: &mut HashMap<&'a str, isize>, avail: &mut HashSet<&'a str>) -> isize {
        let mut max = 0;
        for next in self.neighbours(*state, avail) {
            let (n, t) = if next.a != state.a {
                (next.a, next.time_a)
            } else if next.b != state.b {
                (next.b, next.time_b)
            } else {
                panic!("RIP")
            };

            activated.insert(n, t);
            avail.remove(n);
            max = max.max(self.search_2(&next, activated, avail));
            avail.insert(n);
            activated.remove(n);
        }

        if max != 0 {
            max
        } else {
            self.compute_score(activated)
        }
    }
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin())?;

    // PART 1
    let map: HashMap<String, Node> = input.lines().map(|l| parse_valve(l).unwrap().1).collect();

    let sg = SearchGraph::from(&map);

    let mut acts = HashMap::new();
    let r = sg.search("AA", 30, &mut acts);

    println!("{r}");

    // PART 2
    let s0 = SearchState {
        a: "AA",
        b: "AA",
        time_a: 26,
        time_b: 26,
    };
    let mut acts = HashMap::new();
    let mut avail = sg.values.keys().cloned().collect();
    let r = sg.search_2(&s0, &mut acts, &mut avail);

    println!("{r}");
    Ok(())
}
