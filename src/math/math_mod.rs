/// fast modular exponentiation
///
/// * `r` - radix
/// * `n` - exp
/// * `m` - mod
pub fn mod_pow(mut r: u64, mut n: u64, m: u64) -> u64 {
    let mut ret = 1;
    while n > 0 {
        if n & 1 == 1 {
            ret = ret * r % m;
        }
        r = r * r % m;
        n >>= 1;
    }
    return ret;
}

/// combination for fermat's little theorem
///
/// * `n` - n for nCk
/// * `k` - k for nCk
/// * `m` - mod
fn mod_combi(n: u64, k: u64, m: u64) -> u64 {
    let mut numerator: u64 = 1;
    let mut denominator: u64 = 1;
    for i in 0..k {
        numerator = numerator * (n - i) % m;
        denominator = denominator * (i + 1) % m;
    }
    numerator * mod_pow(denominator, m - 2, m) % m
}

#[cfg(test)]
mod tests {
    use crate::math::math_mod::mod_pow;
    use crate::math::math_mod::mod_combi;

    const MOD: u64 = 1000_000_007;   // prime number

    struct TestCase {
        input: Vec<u64>,
        expect: u64,
    }

    #[test]
    fn test_mod_pow() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                input: vec![2, 0],
                expect: 1,
            },
            TestCase {
                input: vec![3, 4],
                expect: 81,
            },
            TestCase {
                input: vec![2, 10],
                expect: 1024,
            },
            TestCase {
                input: vec![2, 100],
                expect: 976371285,
            },
        ];
        for t in &test_cases {
            let actual = mod_pow(t.input[0], t.input[1], MOD);
            assert_eq!(t.expect, actual);
        }
    }

    #[test]
    fn test_mod_combi() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                input: vec![2, 0],
                expect: 1,
            },
            TestCase {
                input: vec![5, 3],
                expect: 10,
            },
            TestCase {
                input: vec![1000_000_000, 1000],
                expect: 624274358,
            },
        ];
        for t in &test_cases {
            let actual = mod_combi(t.input[0], t.input[1], MOD);
            assert_eq!(t.expect, actual);
        }
    }
}