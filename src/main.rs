#![allow(unused_features)]
#![feature(test)]
#[cfg(test)]
extern crate test;

mod placement;
mod word_placements;
mod grid;
mod crossword;
mod generate;
#[cfg(test)]
mod bench;

use generate::Generator;

fn main() {
    let gen = Generator::new(vec![
        "simulation",
        "algorithm",
        "structure",
        "network",
        "crossword",
        "unicode",
        "monospace",
        "information",
        "concurrent",
        "parallelism"
    ], 1);
    let iter = gen.iter();

    // println!("{}", iter.count());
    for crossword in iter {
        // println!("{}", crossword);
        println!("{:?}", crossword);
    }
}
