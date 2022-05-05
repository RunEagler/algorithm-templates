pub fn round_up(a: u64, b: u64) -> u64{
    (a + b - 1) / b
}

#[cfg(test)]
mod tests {
    use crate::math::math::round_up;

    struct TestCase {
        input: Vec<u64>,
        expect: u64,
    }

    #[test]
    fn test_round_up() {
        let test_cases = vec![
            TestCase {
                input: vec![30, 6],
                expect: 5,
            },
            TestCase {
                input: vec![30, 4],
                expect: 8,
            },
        ];
        for t in &test_cases {
            let actual = round_up(t.input[0], t.input[1]);
            assert_eq!(t.expect, actual);
        }
    }
}