pub struct Solution;

const MEM_SIZE: usize = 38;
static tribonacci_mem: [i32; MEM_SIZE] = [
    0, 1, 1, 2, 4, 7, 13, 24, 44, 81, 149, 274, 504, 927, 1705, 3136, 5768, 10609, 19513, 35890,
    66012, 121415, 223317, 410744, 755476, 1389537, 2555757, 4700770, 8646064, 15902591, 29249425,
    53798080, 98950096, 181997601, 334745777, 615693474, 1132436852, 2082876103,
];

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        tribonacci_mem[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_1137_ex1() {
        assert_eq!(Solution::tribonacci(4), 4);
    }
    #[test]
    fn problem_1137_ex2() {
        assert_eq!(Solution::tribonacci(25), 1389537);
    }
    #[test]
    fn problem_1137_ex3() {
        assert_eq!(Solution::tribonacci(0), 0);
    }
    #[test]
    fn problem_1137_ex4() {
        assert_eq!(Solution::tribonacci(37), 2082876103);
    }
}
