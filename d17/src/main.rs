#![allow(unused_imports)]

use std::collections::{HashMap, HashSet};
use std::ops::Range;

use eyre::Result;
use itertools::Itertools;

type Coord = (isize, isize);


const SHAPES: &[&[Coord]] = &[
    &[(0,0), (0,1), (0,2), (0,3)],
    &[(0,1), (1,0), (1,1), (1,2), (2,1)],
    &[(0,0), (0,1), (0,2), (1,2), (2,2)],
    &[(0,0), (1,0), (2,0), (3,0)],
    &[(0,0), (0,1), (1,0), (1,1)],
];

#[derive(Clone, Copy)]
enum Dir {
    Left = -1,
    Right = 1,
}

fn jets(s: &str) -> impl Iterator<Item = Dir> + '_ {
    s.trim().chars().map(|c| match c {
        '<' => Dir::Left,
        '>' => Dir::Right,
        c => panic!("{c}"),
    }).cycle()
}

#[derive(Default)]
struct Game {
    map: HashSet<Coord>,
    bounds: Range<isize>,
    top: isize,
    active_piece: Option<Tetris>,
}

#[derive(Clone, Copy)]
struct Tetris {
    shape: &'static [Coord],
    anchor: Coord, // bottom left
}

impl Game {
    fn new(bounds: Range<isize>) -> Self { Self { bounds, top: 0, ..Default::default() } }

    fn run(&mut self, jets: impl Iterator<Item = Dir>, n: usize) {
        let mut i = 0;
        
        for d in jets {
            if let None = self.active_piece {
                if i == n {
                    return;
                }
                self.active_piece = Some(Tetris { shape: SHAPES[i % SHAPES.len()], anchor: (self.top + 4, 2) });
                i += 1;
            }

            println!("{}[2J", 27 as char);
            self.print();
            std::thread::sleep(std::time::Duration::from_millis(10));

            
            // Shift
            let piece = self.active_piece.unwrap();
            let try_pos = Tetris {
                shape: piece.shape,
                anchor: (piece.anchor.0, piece.anchor.1 + d as isize),
            };

            if self.is_clear(&try_pos) {
                self.active_piece = Some(try_pos);
            }

            // Drop
            let piece = self.active_piece.unwrap();
            let try_pos = Tetris {
                shape: piece.shape,
                anchor: (piece.anchor.0 - 1, piece.anchor.1),
            };

            if self.is_clear(&try_pos) {
                self.active_piece = Some(try_pos);
            } else {
                self.set_tetris(&piece);
                self.active_piece.take();
            }
        }
    }

    fn set_tetris(&mut self, t: &Tetris) {
        t.shape.iter()
            .map(|offs| (offs.0 + t.anchor.0, offs.1 + t.anchor.1))
            .for_each(|c| { assert!(self.map.insert(c)); });

        self.top = self.top.max(t.shape.iter()
            .map(|offs| offs.0 + t.anchor.0)
            .max()
            .unwrap());
    }

    fn is_clear(&self, t: &Tetris) -> bool {
        t.shape.iter()
            .map(|offs| (offs.0 + t.anchor.0, offs.1 + t.anchor.1))
            .all(|c| c.0 > 0 && self.bounds.contains(&c.1) && !self.map.contains(&c))
    }

    fn print(&self) {
        let active = self.active_piece.map(|t| t.shape.iter()
                .map(|offs| (offs.0 + t.anchor.0, offs.1 + t.anchor.1))
                .collect::<HashSet<_>>())
            .unwrap_or_default();

        let bottom = 1.max(self.top - 25);
        let top = self.top + 5;
        let h_slice = (bottom..top).rev();
        for i in h_slice {
            print!("|");
            for j in self.bounds.clone() {
                if self.map.contains(&(i, j)) {
                    print!("#");
                } else if active.contains(&(i, j)) {
                    print!("@")
                } else {
                    print!(".")
                }
            }
            println!("|");
        }
        if bottom == 1 {
            println!("+-------+");
        }
    }
}

// |..@....|
// |.@@@...|
// |..@....|
// |.......|
// |..####.|
// +-------+


fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin())?;

    // PART 1
    let jets = jets(&input);
    let mut map = Game::new(0..7);

    map.run(jets, 2022);

    let r = map.top;

    map.print();

    println!("{r}");

    // PART 2


    // println!("{r}");
    Ok(())
}