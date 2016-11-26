use test;

use generate::Generator;

#[bench]
fn generate_1(b: &mut test::Bencher) {
    b.iter(|| {
        let gen = Generator::new(vec![
            "toon",
            "took",
            "noob",
            "koob"
        ], 0);
        for cw in gen.iter() {
            let _ = cw;
        }
    })
}

#[bench]
fn generate_2(b: &mut test::Bencher) {
    b.iter(|| {
        let gen = Generator::new(vec![
            "scent",
            "scarf",
            "fleet",
            "tenet",
            "eerie"
        ], 0);
        for cw in gen.iter() {
            let _ = cw;
        }
    })
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
fn generate_3(b: &mut test::Bencher) {
    b.iter(|| {
        let gen = Generator::new(vec![
            "monospace",
            "aesthetics",
            "corporate",
            "vaporwave",
            "crossword",
            "unicode"
        ], 1);
        for cw in gen.iter() {
            let _ = cw;
        }
    })
}
