pub struct Solution;

impl Solution {
    pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
        let n = mountain.len();
        let mut peaks = vec![];
        if n < 1 {
            return peaks;
        }

        let mut i = 1;
        while i < n - 1 {
            if mountain[i - 1] < mountain[i] && mountain[i] > mountain[i + 1] {
                peaks.push(i as i32);
                i += 2;
            } else {
                i += 1;
            }
        }

        return peaks;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_2951_ex1() {
        assert_eq!(Solution::find_peaks(vec![2, 4, 4]), vec![]);
    }

    #[test]
    fn problem_2951_ex2() {
        assert_eq!(Solution::find_peaks(vec![1, 4, 3, 8, 5]), vec![1, 3]);
    }

    #[test]
    fn problem_2951_ex3() {
        assert_eq!(Solution::find_peaks(vec![]), vec![]);
    }

    #[test]
    fn problem_2951_ex4() {
        assert_eq!(Solution::find_peaks(vec![1, 2]), vec![]);
    }
}
