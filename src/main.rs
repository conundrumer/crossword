#![feature(conservative_impl_trait)]
#![allow(unused_features)]
#![feature(test)]
#[cfg(test)]
extern crate test;

mod rand;
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

use std::io;
use std::io::prelude::*;
use std::env;

use generate::Generator;

fn main() {
    let flags = vec!["-n", "-s", "-t"];
    let mut arg_vals = vec![None; flags.len()];
    let args: Vec<_> = env::args().collect();
    for pair in args.windows(2) {
        let (option, arg) = (pair[0].clone(), pair[1].clone());
        for i in 0..arg_vals.len() {
            if &*option == flags[i] {
                match arg.parse::<u64>() {
                    Ok(n) => { arg_vals[i] = Some(n) },
                    Err(e) => {
                        println!("-s {}: {}", arg, e);
                        return
                    }
                }
            }
        }
    }
    let num_areas = arg_vals[0].unwrap_or(0) as usize;
    let seed = arg_vals[1].unwrap_or(0) as u64;
    let num_iters = arg_vals[2].unwrap_or(1) as usize;

    let stdin = io::stdin();
    let words = stdin.lock().lines()
        .map(|line| line.unwrap())
        .take_while(|line| line.len() > 0)
        .collect::<Vec<_>>();
    let words = words.iter().map(|s| s).collect();
    let gen = Generator::new(words, num_areas, seed);
    println!("{}", gen);
    for crossword in gen.multi_iter(num_iters) {
        println!("{}", crossword);
    }
}
