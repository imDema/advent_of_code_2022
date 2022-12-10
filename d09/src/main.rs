use std::collections::HashSet;

use eyre::Result;

enum Move {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl Move {
    pub fn new(s: &str) -> Self {
        let n = s[2..].parse().unwrap();
        match &s[..1] {
            "U" => Self::Up(n),
            "D" => Self::Down(n),
            "L" => Self::Left(n),
            "R" => Self::Right(n),
            _ => panic!("Invalid move")
        }
    }

    pub fn pop(&mut self) -> bool {
        match self {
            Move::Up(v) | Move::Down(v) | Move::Left(v) | Move::Right(v) => {
                *v = v.saturating_sub(1);
                *v > 0
            }
        }
    }

    pub fn dir(&self) -> [isize; 2] {
        match self {
            Move::Up(_) => [0, 1],
            Move::Down(_) => [0, -1],
            Move::Left(_) => [-1, 0],
            Move::Right(_) => [1, 0],
        }
    }
}

struct Rope<const D: usize> {
    pos: [isize; D],
    covered: HashSet<[isize; D]>,
    next: Option<Box<Rope<D>>>,
}

impl<const D: usize> Default for Rope<D> {
    fn default() -> Self {
        let mut covered = HashSet::default();
        covered.insert([0; D]);
        Self { pos: [0; D], covered, next: Default::default() }
    }
}

impl Rope<2> {
    pub fn step(&mut self, mut m: Move) {
        self.pos[0] += m.dir()[0];
        self.pos[1] += m.dir()[1];
        self.covered.insert(self.pos);
        self.next.as_mut().map(|n| n.catch_up(self.pos));
        if m.pop() {
            self.step(m);
        }
    }

    fn catch_up(&mut self, cur: [isize; 2]) {
        let (dx, dy) = (cur[0] - self.pos[0], cur[1] - self.pos[1]);
        
        if dx.abs() > 1 || dy.abs() > 1 {
            let dx = if dx != 0 { dx / dx.abs() } else { 0 };
            let dy = if dy != 0 { dy / dy.abs() } else { 0 };
            
            self.pos[0] += dx;
            self.pos[1] += dy;
        
            self.covered.insert(self.pos);
            self.next.as_mut().map(|n| n.catch_up(self.pos));
        }
    }

    pub fn grow(&mut self) {
        match &mut self.next {
            Some(n) => n.grow(),
            None => self.next = Some(Box::default()),
        }
    }

    pub fn tail(&self) -> &Rope<2> {
        match &self.next {
            Some(n) => n.tail(),
            None => &self,
        }
    }
}


fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin())?;

    // PART 1
    let mut rope = Rope::<2>::default();
    rope.grow();
    
    let moves = input.lines().map(Move::new);
    moves.for_each(|m| rope.step(m));

    let r = rope.tail().covered.len();

    println!("{r}");

    // PART 2
    let mut rope = Rope::<2>::default();
    (0..9).for_each(|_| rope.grow());

    let moves = input.lines().map(Move::new);
    moves.for_each(|m| rope.step(m));

    let r = rope.tail().covered.len();

    println!("{r}");
    Ok(())
}
