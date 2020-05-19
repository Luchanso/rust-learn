pub fn calc_prime(n: usize) -> Vec<usize> {
    let mut init_numbers: Vec<usize> = (0..n + 1).collect();
    init_numbers[1] = 0;
    let mut result: Vec<usize> = Vec::new();

    let mut i = 2;
    while i <= n {
        if init_numbers[i] != 0 {
            result.push(init_numbers[i]);

            for j in (i..n + 1).step_by(i) {
                init_numbers[j] = 0;
            }
        }
        i += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::calc_prime;
    extern crate test;
    use test::Bencher;

    #[test]
    fn should_return_prime_number_before_ten() {
        assert_eq!(calc_prime(10), vec![2, 3, 5, 7]);
    }

    #[test]
    fn should_return_25_prime_numbers() {
        assert_eq!(calc_prime(100).len(), 25);
    }

    #[test]
    fn should_correct_works_on_1000_primes_numbers() {
        calc_prime(1000).len();
    }

    #[bench]
    fn benchmark_for_10_elements(bencher: &mut Bencher) {
        bencher.iter(|| calc_prime(10));
    }

    #[bench]
    fn benchmark_for_1000_elements(bencher: &mut Bencher) {
        bencher.iter(|| calc_prime(1000));
    }

    #[bench]
    fn benchmark_for_million_elements(bencher: &mut Bencher) {
        bencher.iter(|| calc_prime(1_000_000));
    }
}
