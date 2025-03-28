pub struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let m = queries.len();
        let mut distance = vec![0; n];
        let mut graph = vec![Vec::new(); n];
        for i in 0..n {
            distance[i] = i as i32;
            if i != n - 1 {
                graph[i].push(i + 1);
            }
        }
        let mut res = vec![distance[n - 1] as i32; m];
        for i in 0..m {
            let u = queries[i][0] as usize;
            let v = queries[i][1] as usize;
            if v != u + 1 {
                if distance[v] > distance[u] + 1 {
                    graph[u].push(v);
                    distance[v] = distance[u] + 1;
                    let mut priority_queue = BinaryHeap::new();
                    priority_queue.push((v - u, (distance[v], v)));

                    while let Some((_, (d, from))) = priority_queue.pop() {
                        if d != distance[from] {
                            continue;
                        }
                        for &to in &graph[from] {
                            if distance[to] > distance[from] + 1 {
                                distance[to] = distance[from] + 1;
                                priority_queue.push((to-from,(distance[to], to)));
                            }
                        }
                    }
                }
            }
            res[i] = distance[n - 1];
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_3243_ex1() {
        assert_eq!(
            Solution::shortest_distance_after_queries(5, vec![vec![2, 4], vec![0, 2], vec![0, 4]]),
            vec![3, 2, 1]
        );
    }

    #[test]
    fn problem_3243_ex2() {
        assert_eq!(
            Solution::shortest_distance_after_queries(4, vec![vec![0, 3], vec![0, 2]]),
            vec![1, 1]
        );
    }
}
