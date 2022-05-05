use std::collections::HashMap;
use std::hash::Hash;

pub fn count_map<T>(values: Vec<T>) -> HashMap<T,usize> where T: Eq + Hash + Copy{
    values.iter().fold(HashMap::new(),|mut hash,v|{
        *hash.entry(*v).or_insert(0) += 1;
        hash
    })
}

#[cfg(test)]
mod tests {
    use crate::data_processing::hashmap::count_map;
    use std::collections::HashMap;

    #[test]
    fn test_count_map() {
        struct TestCase<T> {
            input: Vec<T>,
            expect: HashMap<T,usize>,
        }
        let test_cases = vec![
            TestCase { input: vec![1,5,3,2,3,5,5], expect: HashMap::from([(1,1),(2,1),(3,2),(5,3)]) },
        ];
        for t in test_cases {
            let actual = count_map(t.input);
            assert_eq!(t.expect, actual);
        }
    }
}