mod placement;
mod grid;
mod word;
mod crossword;
mod generate;
#[cfg(test)]
mod tests;

use generate::{ Generator, SimpleGenerator };

fn main() {
    let crosswords = <SimpleGenerator as Generator>::generate(vec![
        "toon",
        "took",
        "noob",
        "koob"
    ], (1, 5, 5));
    println!("{}", crosswords.len());
    for ref crossword in crosswords.iter().take(100) {
        println!("{}", crossword);
    }
}
