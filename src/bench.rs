use test;

use generate::Generator;

#[bench]
fn generate(b: &mut test::Bencher) {
    b.iter(|| Generator::generate(vec![
        "toon",
        "took",
        "noob",
        "koob"
    ], (1, 5)))
}

#[bench]
fn generate_medium(b: &mut test::Bencher) {
    b.iter(|| Generator::generate(vec![
        "scent",
        "scarf",
        "fleet",
        "tenet",
        "eerie"
    ], (1, 5)))
}

/*
　　　　　　ｕ
　　　　　　ｎ
　　　ｃ　　ｉ
　　　ｒ　　ｃ
　　ｃｏｒｐｏｒａｔｅ
　ｖ　ｓ　　ｄ
　ａｅｓｔｈｅｔｉｃｓ
　ｐ　ｗ
ｍｏｎｏｓｐａｃｅ
　ｒ　ｒ
　ｗ　ｄ
　ａ
　ｖ
　ｅ
 */
#[bench]
#[ignore]
fn generate_huge(b: &mut test::Bencher) {
    b.iter(|| Generator::generate(vec![
        "monospace",
        "aesthetics",
        "corporate",
        "vaporwave",
        "crossword",
        "unicode"
    ], (1, 5)))
}
