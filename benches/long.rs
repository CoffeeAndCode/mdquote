#![cfg_attr(feature = "nightly", feature(test))]

#[cfg(all(feature = "nightly", test))]
extern crate test;

#[cfg(all(feature = "nightly", test))]
mod benchmarks {
    use mdquote::add_quotes;
    use std::io::Cursor;
    use test::Bencher;

    const ITERATIONS: usize = 5_000;

    #[bench]
    fn bench_fast(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(ITERATIONS);

            (0..n).for_each(|num| {
                let output = Vec::<u8>::new();
                add_quotes(Cursor::new(num.to_string()), output, true).unwrap();
            });
        });
    }

    #[bench]
    fn bench_slow(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(ITERATIONS);

            (0..n).for_each(|num| {
                let output = Vec::<u8>::new();
                add_quotes(Cursor::new(num.to_string()), output, false).unwrap();
            });
        });
    }
}
