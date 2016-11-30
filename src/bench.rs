use test;

use generate::tests::test_generator;

#[bench]
fn generate_1(b: &mut test::Bencher) {
    b.iter(|| {
        let words =  vec![
            "toon",
            "took",
            "noob",
            "koob"
        ];
        test_generator(words, 0, &|gen| {
            for cw in gen.iter() {
                let _ = cw;
            }
        });
    })
}

#[bench]
fn generate_2(b: &mut test::Bencher) {
    b.iter(|| {
        let words =  vec![
            "scent",
            "scarf",
            "fleet",
            "tenet",
            "eerie"
        ];
        test_generator(words, 0, &|gen| {
            for cw in gen.iter() {
                let _ = cw;
            }
        });
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
        let words = vec![
            "monospace",
            "aesthetics",
            "corporate",
            "vaporwave",
            "crossword",
            "unicode"
        ];
        test_generator(words, 1, &|gen| {
            for cw in gen.iter() {
                let _ = cw;
            }
        });
    })
}
