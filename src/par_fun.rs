
#[cfg(test)]
mod tests {
    use rayon::prelude::*;
    use test::{self, Bencher};

    #[bench]
    fn bench_xor_1_000_seq(b: &mut Bencher) {
        b.iter(|| (0..1_000).fold(0, |a, b| a ^ b));
    }

    #[bench]
    fn bench_xor_1_000_par(b: &mut Bencher) {
        ::rayon::ThreadPool::new(::rayon::Configuration::new()).unwrap().install(|| {
            b.iter(|| (0..1_000).into_par_iter().reduce(|| 0, |a, b| a ^ b));
        });
    }
    #[bench]
    fn bench_xor_1_000_000_seq(b: &mut Bencher) {
        b.iter(|| (0..1_000_000).fold(0, |a, b| a ^ b));
    }

    #[bench]
    fn bench_xor_1_000_000_par(b: &mut Bencher) {
        ::rayon::ThreadPool::new(::rayon::Configuration::new()).unwrap().install(|| {
            b.iter(|| (0..1_000_000).into_par_iter().reduce(|| 0, |a, b| a ^ b));
        });
    }

    #[bench]
    fn bench_xor_5_000_000_seq(b: &mut Bencher) {
        b.iter(|| (0..5_000_000).fold(0, |a, b| a ^ b));
    }

    #[bench]
    fn bench_xor_5_000_000_par(b: &mut Bencher) {
        ::rayon::ThreadPool::new(::rayon::Configuration::new()).unwrap().install(|| {
            b.iter(|| (0..5_000_000).into_par_iter().reduce(|| 0, |a, b| a ^ b));
        });
    }
}
