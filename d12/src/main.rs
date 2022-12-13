#![allow(unused_imports)]

use eyre::Result;
use itertools::Itertools;
use ndarray::Array2;

use petgraph::algo::dijkstra;
use petgraph::data::FromElements;
use petgraph::prelude::GraphMap;
use petgraph::{Directed, Graph, Undirected};

struct Map {
    grid: Array2<u8>,
}

impl Map {
    pub fn parse(s: &str) -> (Self, (usize, usize), (usize, usize)) {
        let w = s.find('\n').unwrap();
        let mut start = None;
        let mut end = None;
        let vec: Vec<u8> = s
            .chars()
            .filter(|&c| c != '\n')
            .enumerate()
            .map(|(i, c)| match c {
                c @ 'a'..='z' => c as u8 - b'a',
                'S' => {
                    start = Some(i);
                    0
                }
                'E' => {
                    end = Some(i);
                    26
                }
                _ => panic!("invalid char in map"),
            })
            .collect();
        let h = vec.len() / w;

        let s = Self {
            grid: Array2::from_shape_vec((h, w), vec).unwrap(),
        };

        let start = start.map(|s| (s / w, s % w)).unwrap();
        let end = end.map(|e| (e / w, e % w)).unwrap();
        (s, start, end)
    }

    pub fn compute_connections(
        &self,
        cond: impl Fn(u8, u8) -> bool,
    ) -> GraphMap<(usize, usize), i32, Directed> {
        let dims = self.grid.raw_dim();
        let cond = &cond;
        let iter = self.grid.indexed_iter().flat_map(|(i, &v)| {
            [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .into_iter()
                .filter_map(move |(dx, dy)| {
                    let x = i.0 as isize + dx;
                    let y = i.1 as isize + dy;
                    if x < 0 || y < 0 || x >= dims[0] as isize || y >= dims[1] as isize {
                        return None;
                    }
                    let (x, y) = (x as usize, y as usize);
                    if (cond)(self.grid[[x, y]], v) {
                        Some((i, (x, y)))
                    } else {
                        None
                    }
                })
        });

        GraphMap::<_, _, Directed>::from_edges(iter)
    }
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin())?;

    // PART 1
    let (map, s, e) = Map::parse(&input);
    eprintln!("s: {s:?}, e: {e:?}");

    let graph = map.compute_connections(|a, b| b - a <= 1);
    let res = dijkstra(&graph, s, Some(e), |_| 1);
    
    let r = res[&e];
    println!("{r}");

    // PART 2
    let graph = map.compute_connections(|a, b| a - b <= 1);
    let res = dijkstra(&graph, e, None, |_| 1);

    let r = res
        .into_iter()
        .filter(|r| map.grid[r.0] == 0)
        .min_by_key(|t| t.1)
        .unwrap()
        .1;

    println!("{r}");
    Ok(())
}
