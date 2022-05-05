use std::collections::BinaryHeap;

/// dijkstra
///
/// * `start` - target vertex
/// * `n` - vertex size
/// * `edges` - [(from, to, cost)]
pub fn dijkstra(start: usize, n: usize, edges: Vec<(usize, usize, usize)>) -> Vec<usize> {
    let mut queue = BinaryHeap::new();
    let mut graph = vec![vec![]; n];
    let mut dist = vec![1 << 30; n];
    for es in edges {
        graph[es.0].push((es.1, es.2));
    }

    dist[start] = 0;
    queue.push((0, 0));

    while let Some((v, d)) = queue.pop() {
        if dist[v] < d {
            continue;
        }
        for (next, c) in &graph[v] {
            if dist[*next] > dist[v] + c {
                dist[*next] = dist[v] + c;
                queue.push((*next, dist[*next]));
            }
        }
    }
    dist
}

#[cfg(test)]
mod tests {
    use crate::graph::dijkstra::dijkstra;

    struct TestCase {
        input: Vec<(usize, usize, usize)>,
        expect: usize,
    }

    #[test]
    fn test_dijkstra() {
        let test_cases = vec![TestCase {
            input: vec![
                (0, 1, 1),
                (1, 0, 1),
                (1, 2, 3),
                (2, 0, 4),
            ],
            expect: 4,
        }];

        for t in test_cases {
            let actual = dijkstra(0, 3, t.input);
            assert_eq!(t.expect, actual[2]);
        }
    }
}