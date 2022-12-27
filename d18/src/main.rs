#![allow(unused_imports)]

use std::collections::{HashSet, BTreeMap, HashMap, BTreeSet};
use std::ops::Range;

use eyre::Result;
use itertools::Itertools;
use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::sequence::separated_pair;

type Coord = (isize, isize, isize);

fn parse_coord(s: &str) -> IResult<&str, Coord> {
    let (s, a) = digit1(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, b) = digit1(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, c) = digit1(s)?;

    Ok((s, (a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap())))
}

fn offset (a: Coord, b: Coord) -> Coord {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

const NEIGH: &[Coord] = &[
    (1,0,0), (0,1,0), (0,0,1),
    (-1,0,0), (0,-1,0), (0,0,-1),
];

const OUTER: usize = 0;

/// Computes the connected components in the voxel neighbour graph
struct Components {
    clusters: BTreeMap<usize, Vec<Coord>>,
    voxels: HashMap<Coord, usize>,
    bx: Range<isize>,
    by: Range<isize>,
    bz: Range<isize>,
}

impl Components {
    pub fn new(map: &HashSet<Coord>) -> Self {
        // Compute bounding box + 1
        let mut pts = map.iter();
        let first = pts.next().unwrap();
        let mut bx = first.0 - 1..first.0 + 2;
        let mut by = first.1 - 1..first.1 + 2;
        let mut bz = first.2 - 1..first.2 + 2;
        for p in pts {
            bx.start = bx.start.min(p.0 - 1);
            by.start = by.start.min(p.1 - 1);
            bz.start = bz.start.min(p.2 - 1);
            bx.end = bx.end.max(p.0 + 2);
            by.end = by.end.max(p.1 + 2);
            bz.end = bz.end.max(p.2 + 2);
        }
        // Assign each voxel to a new component
        let mut id = 1;
        let mut components = BTreeMap::new();
        let mut vox = HashMap::new();
        for i in bx.clone() {
            for j in by.clone() {
                for k in bz.clone() {
                    if !map.contains(&(i,j,k)) {
                        components.insert(id, vec![(i, j, k)]);
                        vox.insert((i, j, k), id);
                        id += 1;
                    }
                }
            }
        }
        Self { clusters: components, voxels: vox, bx, by, bz }
    }

    // Compute connected components in the graph
    fn compact(&mut self) {
        let mut queue: BTreeSet<Coord> = self.voxels.keys().cloned().collect();
        while let Some(cur) = queue.pop_last() {
            let mut cmp = self.voxels[&cur];
            for neigh in NEIGH.iter().map(|o| offset(cur, *o)) {
                if cmp != OUTER && (!self.bx.contains(&neigh.0) || !self.by.contains(&neigh.1) || !self.bz.contains(&neigh.2)) {
                    let old_cmp = self.clusters.remove(&cmp).unwrap();
                    old_cmp.iter().for_each(|c| { self.voxels.insert(*c, OUTER); });
                    queue.extend(old_cmp.iter().cloned());
                    self.clusters.entry(OUTER).or_default().extend(old_cmp);
                    cmp = OUTER;
                } else if let Some(&nc) = self.voxels.get(&neigh) {
                    if nc > cmp {
                        let old_cmp = self.clusters.remove(&nc).unwrap();
                        old_cmp.iter().for_each(|c| { self.voxels.insert(*c, cmp); });
                        queue.extend(old_cmp.iter().cloned());
                        self.clusters.get_mut(&cmp).unwrap().extend(old_cmp);
                    }
                }
            }
        }
    }

    fn get_component(&self, id: usize) -> Option<HashSet<Coord>> {
        self.clusters.get(&id).map(|v| v.iter().cloned().collect())
    }
}

fn surface(map: &HashSet<Coord>) -> usize {
    map.iter()
        .flat_map(|c| NEIGH.iter().map(|o| offset(*c, *o)))
        .filter(|s| !map.contains(s))
        .count()
}

fn surface_2(solid: &HashSet<Coord>, void: &HashSet<Coord>) -> usize {
    void.iter()
        .flat_map(|c| NEIGH.iter().map(|o| offset(*c, *o)))
        .filter(|s| solid.contains(s))
        .count()
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin())?;

    // PART 1
    let map: HashSet<Coord> = input.lines().map(|l| parse_coord(l).unwrap().1).collect();

    let r = surface(&map);

    println!("{r}");

    // PART 2
    let mut c = Components::new(&map);
    c.compact();

    let void = c.get_component(0).unwrap();

    let r = surface_2(&map, &void);

    println!("{r}");

    Ok(())
}
