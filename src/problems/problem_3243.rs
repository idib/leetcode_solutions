use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut distance = vec![0; n];
        let mut inverse_graph = vec![Vec::new(); n];
        for i in 0..n {
            distance[i] = n - i - 1;
            if i != 0 {
                inverse_graph[i].push(i - 1);
            }
        }
        let mut res = Vec::new();
        for query in queries {
            let u = query[0] as usize;
            let v = query[1] as usize;

            inverse_graph[v].push(u);

            if distance[v] + 1 < distance[u] {
                distance[u] = distance[v] + 1;
                let mut queue = VecDeque::new();
                queue.push_back(u);

                while let Some(cur) = queue.pop_front() {
                    for &parent in &inverse_graph[cur] {
                        if distance[cur] + 1 < distance[parent] {
                            distance[parent] = distance[cur] + 1;
                            queue.push_back(parent);
                        }
                    }
                }
            }

            res.push(distance[0] as i32);
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
