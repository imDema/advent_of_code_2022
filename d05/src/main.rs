use eyre::Result;
use itertools::Itertools;
use ndarray::Array2;

struct Command {
    n: usize,
    from: usize,
    to: usize,
}

impl Command {
    pub fn parse_str(s: &str) -> Self {
        let (n, from, to) = s
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .map(|w| w.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Self {
            n,
            from: from - 1,
            to: to - 1,
        }
    }
}

fn parse_setup(setup: &str) -> Vec<Vec<char>> {
    let width = setup.lines().next().unwrap().len();
    let height = setup.lines().count();
    let char_vec = setup.chars().filter(|&c| c != '\n').collect_vec();
    let s: Array2<char> = Array2::from_shape_vec((height, width), char_vec).unwrap();

    s.columns()
        .into_iter()
        .skip(1)
        .step_by(4)
        .map(|r| {
            r.into_iter()
                .rev()
                .skip(1)
                .take_while(|&&c| c != ' ')
                .cloned()
                .collect()
        })
        .collect()
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin())?;

    let (setup, commands) = input.split("\n\n").collect_tuple().unwrap();

    let mut state = parse_setup(setup);
    let commands = commands.lines().map(Command::parse_str);

    for Command { n, from, to } in commands {
        for _ in 0..n {
            let el = state[from].pop().unwrap();
            state[to].push(el);
        }
    }

    let r = state.iter().map(|l| *l.last().unwrap()).collect::<String>();

    println!("{r}");

    // PART 2

    let (setup, commands) = input.split("\n\n").collect_tuple().unwrap();

    let mut state = parse_setup(setup);
    let commands = commands.lines().map(Command::parse_str);

    for Command { n, from, to } in commands {
        let start = state[from].len() - n;
        let stack: Vec<_> = state[from].drain(start..).collect();
        state[to].extend(stack);
    }

    let r = state.iter().map(|l| *l.last().unwrap()).collect::<String>();

    println!("{r}");

    Ok(())
}
