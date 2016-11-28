#![feature(conservative_impl_trait)]
#![allow(unused_features)]
#![feature(test)]
#[cfg(test)]
extern crate test;

mod placement;
mod bounding_box;
mod word_placements;
mod grid;
mod grid_cell;
mod crossword;
mod filter;
mod generate;
#[cfg(test)]
mod bench;

use generate::Generator;

fn main() {
    let gen = Generator::new(vec![
        "benjamin",
        "roytenberg",
        "chase",
        "sapphire",
        "reserve",
        "soylent",
        "slav",
        "leeks",
        "vaporwave",
        "aesthetic",
        "hackathons",
        "memes",
    ], 100);
    let iter = gen.iter();

    // println!("{}", iter.count());
    for crossword in iter {
        println!("{}", crossword);
    }
}
