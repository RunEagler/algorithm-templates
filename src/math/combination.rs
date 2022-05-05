pub struct Combination {
    fact: Vec<u64>,
    finv: Vec<u64>,
    p: u64,
}

impl Combination {
    fn new(n: usize, p: u64) -> Combination {
        let mut fact: Vec<u64> = vec![0; n + 1];
        let mut finv: Vec<u64> = vec![0; n + 1];
        let mut inv: Vec<u64> = vec![0; n + 1];
        fact[0] = 1;
        fact[1] = 1;
        finv[0] = 1;
        finv[1] = 1;
        inv[1] = 1;
        for i in 2..=n {
            fact[i] = fact[i - 1] * i as u64 % p;
            inv[i] = p - inv[(p % i as u64) as usize] * (p / i as u64) % p;
            finv[i] = finv[i - 1] * inv[i] % p;
        }
        Combination {
            fact,
            finv,
            p,
        }
    }

    fn n_c_k(&self, n: usize, k: usize) -> u64 {
        if n < k {
            0
        } else {
            self.fact[n] * (self.finv[k] * self.finv[n - k] % self.p) % self.p
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::combination::Combination;

    const MOD: u64 = 1000_000_007;

    // prime number
    struct TestCase {
        input: Vec<usize>,
        expect: u64,
    }

    #[test]
    fn test_combination() {
        let combi = Combination::new(100000, MOD);
        let test_cases = vec![
            TestCase {
                input: vec![5, 3],
                expect: 10,
            },
            TestCase {
                input: vec![100, 0],
                expect: 1,
            },
            TestCase {
                input: vec![5, 5],
                expect: 1,
            },
            TestCase {
                input: vec![100000, 100],
                expect: 801473732,
            },
        ];
        for t in &test_cases {
            let actual = combi.n_c_k(t.input[0], t.input[1]);
            assert_eq!(actual, t.expect);
        }
    }
}