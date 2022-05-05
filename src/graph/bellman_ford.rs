/// Bellman-Ford algorithm
///
/// * `start` - target vertex
/// * `n` - vertex size
/// * `edges` - [(from, to, cost)]
pub fn bellman_ford(start: usize, n: usize, edges: &Vec<(usize, usize, i64)>) -> (bool, Vec<i64>) {
    let mut dist = vec![std::i64::MAX; n];
    dist[start] = 0;
    for i in 0..n {
        let mut updated = false;
        for es in edges {
            if dist[es.0] != std::i64::MAX && dist[es.0] + es.2 < dist[es.1] {
                dist[es.1] = dist[es.0] + es.2;
                updated = true;
                if i == n - 1 && es.1 == n - 1 {
                    return (true, vec![]);
                }
            }
        }
        if !updated {
            break;
        }
    }
    return (false, dist);
}


#[cfg(test)]
mod tests {
    use crate::graph::bellman_ford::bellman_ford;

    #[test]
    fn test_bellman_ford() {
        let input = vec![
            (0, 1, 1),
            (1, 0, 1),
            (1, 2, 3),
            (2, 0, 4),
        ];
        let (is_close, actual) = bellman_ford(0, 3, &input);
        assert_eq!(4, actual[2]);
    }
}