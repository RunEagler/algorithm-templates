/// Floyd–Warshall Algorithm
///
/// * `n` - vertex size
/// * `edges` - [(from,to,cost)]
pub fn warshall_floyd(n: usize, edges: Vec<(usize, usize, usize)>) -> Vec<Vec<usize>> {
    let mut graph: Vec<Vec<usize>> = vec![vec![1 << 30; n]; n];
    for (a, b, c) in &edges {
        graph[*a][*b] = *c;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                graph[i][j] = if i == j { 0 } else { graph[i][j].min(graph[i][k] + graph[k][j]) };
            }
        }
    }
    return graph;
}

#[cfg(test)]
mod tests {
    use crate::graph::warshall_floyd::warshall_floyd;

    struct TestCase {
        input: Input,
        expect: Vec<Vec<usize>>,
    }

    struct Input {
        n: usize,
        edges: Vec<(usize, usize, usize)>,
    }

    #[test]
    fn test_warshall_floyd() {
        let test_cases = vec![
            TestCase {
                input: Input {
                    n: 3,
                    edges: vec![
                        (0, 1, 1),
                        (1, 0, 2),
                        (1, 2, 3),
                        (2, 0, 4),
                    ],
                },
                expect: vec![
                    vec![0, 1, 4],
                    vec![2, 0, 3],
                    vec![4, 5, 0],
                ],
            },
        ];

        for t in test_cases {
            let actual = warshall_floyd(t.input.n, t.input.edges);
            for (i, vertices) in t.expect.into_iter().enumerate() {
                for (j, v) in vertices.into_iter().enumerate() {
                    assert_eq!(v, actual[i][j]);
                }
            }
        }
    }
}