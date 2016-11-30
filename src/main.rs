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

use std::io;
use std::io::prelude::*;
use std::env;

use generate::Generator;

fn main() {
    let mut num_areas = 0;
    let mut seed = 0;
    let args: Vec<_> = env::args().collect();
    for pair in args.windows(2) {
        let (option, arg) = (pair[0].clone(), pair[1].clone());
        match &*option {
            "-n" => match arg.parse::<usize>() {
                Ok(n) => { num_areas = n },
                Err(e) => {
                    println!("-n {}: {}", arg, e);
                    return
                }
            },
            "-s" => match arg.parse::<usize>() {
                Ok(n) => { seed = n },
                Err(e) => {
                    println!("-s {}: {}", arg, e);
                    return
                }
            },
            _ => {}
        }
    }
    let _ = seed;

    let stdin = io::stdin();
    let words = stdin.lock().lines().map(|line| line.unwrap()).collect::<Vec<_>>();
    let words = words.iter().map(|s| s).collect();
    let gen = Generator::new(words, num_areas);
    let iter = gen.iter()
        .scan(0, |state, cw| {
            if cw.num_overlaps() >= *state {
                *state = cw.num_overlaps();
                return Some(Some(cw))
            }
            Some(None)
        }).filter_map(|x| x);

    // println!("{}", iter.count());
    println!("{}", gen);
    for crossword in iter {
        println!("{}", crossword);
    }
}
