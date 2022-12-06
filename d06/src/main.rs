use std::collections::HashSet;

use eyre::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin())?;

    // PART 1
    let chars = input.chars().collect::<Vec<_>>();
    let r = chars
        .windows(4)
        .find_position(|w| w.iter().cloned().collect::<HashSet<_>>().len() == w.len())
        .unwrap();
    let r = r.0 + 4;

    println!("{r}");

    // PART 2
    let r = chars
        .windows(14)
        .find_position(|w| w.iter().cloned().collect::<HashSet<_>>().len() == w.len())
        .unwrap();
    let r = r.0 + 14;

    println!("{r}");

    // println!("{r}");
    Ok(())
}
