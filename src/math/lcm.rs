use crate::math::gcd::gcd;

/// Least common multiple
///
/// * `a` - num1
/// * `b` - num2
fn lcm(a:usize,b:usize)-> usize{
    return a / gcd(a,b) * b
}

#[cfg(test)]
mod tests {
    use crate::math::lcm::lcm;

    struct TestCase {
        input: Vec<usize>,
        expect: usize,
    }

    #[test]
    fn test_lcm() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                input: vec![4, 6],
                expect: 12,
            },
            TestCase {
                input: vec![11, 9],
                expect: 99,
            },
            TestCase {
                input: vec![36, 6],
                expect: 36,
            },
            TestCase {
                input: vec![1, 1],
                expect: 1,
            },
        ];
        for t in &test_cases {
            let actual = lcm(t.input[0], t.input[1]);
            assert_eq!(t.expect, actual);
        }
    }
}