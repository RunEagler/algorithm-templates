/// Greatest common divisor
///
/// * `p` - num1
/// * `q` - num2
pub fn gcd(p: usize, q: usize) -> usize {
    if q == 0 {
        return p;
    }
    return gcd(q, p % q);
}

#[cfg(test)]
mod tests {
    use crate::math::gcd::gcd;

    struct TestCase {
        input: Vec<usize>,
        expect: usize,
    }

    #[test]
    fn test_gcd() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                input: vec![11, 7],
                expect: 1,
            },
            TestCase {
                input: vec![6, 4],
                expect: 2,
            },
            TestCase {
                input: vec![20, 5],
                expect: 5,
            },
        ];
        for t in &test_cases {
            let actual = gcd(t.input[0], t.input[1]);
            assert_eq!(t.expect, actual);
        }
    }
}