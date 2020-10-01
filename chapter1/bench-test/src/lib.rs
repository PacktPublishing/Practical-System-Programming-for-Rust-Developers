#![feature(test)]

extern crate test;

pub fn get_fact(n: u64) -> u64 {
    if n < 2 {
        1
    } else {
        n * get_fact(n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(120, get_fact(5));
    }

    #[bench]
    fn bench_get_fact(b: &mut Bencher) {
        b.iter(|| get_fact(1234571));
    }
}
