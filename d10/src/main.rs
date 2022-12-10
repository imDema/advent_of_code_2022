use eyre::Result;
use ndarray::Array2;

enum Op {
    Noop,
    Addx(isize),
}

impl Op {
    pub fn parse(s: &str) -> Self {
        match &s[..4] {
            "noop" => Op::Noop,
            "addx" => Op::Addx(s[5..].parse().unwrap()),
            _ => panic!("invalid opcode"),
        }
    }

    pub fn schedule(self) -> (Self, usize) {
        match self {
            Op::Noop => (self, 1),
            Op::Addx(_) => (self, 2),
        }
    }
}

struct Processor<I: Iterator<Item = Op>> {
    code: I,
    rx: isize,
    issuer: Option<(Op, usize)>,
}

impl<I: Iterator<Item = Op>> Processor<I> {
    fn new(code: I) -> Self {
        Self {
            code,
            rx: 1,
            issuer: None,
        }
    }
}

impl<I: Iterator<Item = Op>> Iterator for Processor<I> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        if let None = self.issuer {
            self.issuer = Some(self.code.next().map(Op::schedule)?);
        }
        let (op, mut rem) = self.issuer.take().unwrap();
        
        let ret = self.rx; // Return value before end of execution
        rem -= 1;
        match (op, rem) {
            (Op::Addx(v), 0) => {
                self.rx += v;
            }
            (Op::Noop, 0) => {}
            incomplete => self.issuer = Some(incomplete),
        }
        Some(ret)
    }
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin())?;

    // PART 1
    let instr = input.lines().map(Op::parse);

    let r: isize = Processor::new(instr)
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(i, v)| (i as isize + 1) * v)
        .sum();

    println!("{r}");
    
    // PART 2
    let instr = input.lines().map(Op::parse);

    let mut screen = Array2::from_shape_simple_fn((40, 6), || ' ');
    Processor::new(instr)
        .enumerate()
        .map(|(i, x)| ([i % 40, i / 40], x))
        .for_each(|(coord, x)| if (x - coord[0] as isize).abs() <= 1 { screen[coord] = '#' });

    println!("{}", screen.t());

    // Prettier
    println!();
    screen.t().rows().into_iter().for_each(|r| println!("{}", r.into_iter().collect::<String>()));

    Ok(())
}
