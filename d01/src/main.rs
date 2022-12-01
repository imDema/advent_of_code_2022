use std::{cmp::Reverse, collections::BinaryHeap, io::stdin};

use eyre::Result;

fn main() -> Result<()> {
    let input = std::io::read_to_string(stdin())?;

    // Part 1
    let max = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|l| l.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .max()
        .unwrap();

    println!("{max}");

    // Part 2
    let top = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|l| l.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .fold(BinaryHeap::from(vec![Reverse(0)]), |mut heap, x| {
            if x > heap.peek().unwrap().0 {
                heap.push(Reverse(x));
            }
            if heap.len() > 3 {
                heap.pop();
            }
            heap
        });

    let max = top.into_iter().map(|r| r.0).sum::<usize>();

    println!("{max}");

    Ok(())
}
