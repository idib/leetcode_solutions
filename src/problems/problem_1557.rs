pub struct Solution;

impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut income_nodes: Vec<bool> = vec![false; n as usize];
        income_nodes.resize(n as usize, false);

        for x in edges {
            income_nodes[x[1] as usize] = true;
        }

        for i in 0..income_nodes.len() {
            if !income_nodes[i] {
                result.push(i as i32)
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn problem_1557_ex1() {
        assert_eq!(
            Solution::find_smallest_set_of_vertices(
                6,
                vec![vec![0, 1], vec![0, 2], vec![2, 5], vec![3, 4], vec![4, 2]]
            ),
            vec![0, 3]
        );
    }

    #[test]
    fn problem_1557_ex2() {
        assert_eq!(
            Solution::find_smallest_set_of_vertices(
                5,
                vec![vec![0, 1], vec![2, 1], vec![3, 1], vec![1, 4], vec![2, 4]]
            ),
            vec![0, 2, 3]
        );
    }
}
