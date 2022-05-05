pub fn is_palindrome(s: Vec<char>) -> bool {
    (0..s.len() / 2).all(|i| s[i] == s[s.len() - i - 1])
}

pub fn compress_str(s: Vec<char>) -> String {
    s.iter()
        .fold(vec![(s[0], 0)], |mut ret, c| {
            let last_index = ret.len() - 1;
            if ret[last_index].0 == *c {
                ret[last_index] = (ret[last_index].0, ret[last_index].1 + 1);
            } else {
                ret.push((*c, 1));
            }
            ret
        })
        .iter().map(|v| format!("{}{}", v.0.to_string(), v.1))
        .collect::<Vec<String>>().join("")
}



#[cfg(test)]
mod tests {
    use crate::data_processing::strings::is_palindrome;
    use crate::data_processing::strings::compress_str;

    #[test]
    fn test_is_palindrome() {
        struct TestCase<'a> {
            input: &'a str,
            expect: bool,
        }
        let test_cases = vec![
            TestCase { input: "abcdcba", expect: true },
            TestCase { input: "abcdabc", expect: false },
            TestCase { input: "a", expect: true },
            TestCase { input: "aba", expect: true },
            TestCase { input: "abba", expect: true },
            TestCase { input: "abbbab", expect: false },
            TestCase { input: "pailndrome", expect: false },
        ];
        for t in test_cases {
            let actual = is_palindrome(t.input.chars().collect::<Vec<char>>());
            assert_eq!(t.expect, actual);
        }
    }

    #[test]
    fn test_compress_string() {
        struct TestCase<'a> {
            input: &'a str,
            expect: &'a str,
        }
        let test_cases = vec![
            TestCase { input: "abbccc", expect: "a1b2c3" },
            TestCase { input: "abc", expect: "a1b1c1" },
            TestCase { input: "z", expect: "z1" },
            TestCase { input: "abcaabbcc", expect: "a1b1c1a2b2c2" },
            TestCase { input: "eeeee", expect: "e5" },
        ];
        for t in test_cases {
            let actual = compress_str(t.input.chars().collect::<Vec<char>>());
            assert_eq!(t.expect, actual);
        }
    }
}