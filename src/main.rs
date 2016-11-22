#![allow(unused_features)]
#![feature(test)]
#[cfg(test)]
extern crate test;

mod placement;
mod grid;
mod crossword;
mod generate;
#[cfg(test)]
mod bench;

use generate::Generator;

fn main() {
    let crosswords = Generator::generate(vec![
        "toon",
        "took",
        "noob",
        "koob"
    ], (1, 5));
    println!("{}", crosswords.len());
    for ref crossword in crosswords.iter().take(100) {
        println!("{}", crossword);
    }
}
