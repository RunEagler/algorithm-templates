
/// prime factorization(include 1)
///
/// * `v` - target value
pub fn enum_prime(v: usize) -> Vec<(usize, usize)> {
    let mut i: usize = 2;
    let mut p: usize = v;
    let mut ret: Vec<(usize, usize)> = vec![(1, 1)];
    while i * i <= p {
        if p % i == 0 {
            let mut e_count: usize = 0;
            while p % i == 0 {
                e_count += 1;
                p /= i;
            }
            ret.push((i, e_count));
        }
        i += 1;
    }
    if p > 1 {
        ret.push((p, 1));
    }
    ret
}


#[cfg(test)]
mod tests {
    use crate::math::prime::enum_prime;

    struct TestCase {
        input: usize,
        expect: Vec<(usize, usize)>,
    }

    #[test]
    fn test_enum_prime() {
        let test_cases = vec![
            TestCase {
                input: 1024,
                expect: vec![(1, 1), (2, 10)],
            },
            TestCase {
                input: 405,
                expect: vec![(1, 1), (3, 4), (5, 1)],
            },
            TestCase {
                input: 1000_000_007,
                expect: vec![(1, 1), (1000_000_007, 1)],
            },
        ];

        for t in test_cases {
            let actual = enum_prime(t.input);
            assert_eq!(t.expect.len(), actual.len());
            for (i, (r, p)) in t.expect.into_iter().enumerate() {
                assert_eq!(r, actual[i].0);
                assert_eq!(p, actual[i].1);
            }
        }
    }
}