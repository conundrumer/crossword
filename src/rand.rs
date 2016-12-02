use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub fn hash<T: Hash>(t: T, seed: u64) -> u64 {
    if seed == 0 {
        return 0
    }
    let mut s = DefaultHasher::new();
    (t, seed).hash(&mut s);
    s.finish()
}

// two param 1-1 mapping of a range of ints
pub fn rand_range(n: usize, seed: u64) -> impl Fn(usize) -> usize {
    let (offset, stride) = if n > 1 && seed != 0 {
        let seed = seed as usize;
        (seed % n, (seed / n) % (n - 1) + 1)
    } else {
        (0, 0)
    };
    return move |i| {
        if seed == 0 {
            return i
        }
        let j = i * stride + offset;
        return (j + j / n) % n;
    }
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn rand_range() {
        let map_range = super::rand_range(1, 0);
        assert_eq!(0, map_range(0));

        let map_range = super::rand_range(4, 0);
        for i in 0..4 {
            print!("{} ", map_range(i));
            assert_eq!(i, map_range(i));
        }
        println!();

        let expected = [1, 3, 2, 0];
        let map_range = super::rand_range(4, 1 + 4 * 1);
        for i in 0..4 {
            print!("{} ", map_range(i));
            assert_eq!(expected[i], map_range(i));
        }
        println!();
    }
}
