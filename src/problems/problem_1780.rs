pub struct Solution;

const MEM_SIZE: usize = 15;
static POWERS_OF_THREE: [i32; MEM_SIZE] = [
    1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049, 177147, 531441, 1594323, 4782969,
];

static FULL_SUM_POWERS_OF_THREE: [i32; MEM_SIZE] = [
    1, 4, 13, 40, 121, 364, 1093, 3280, 9841, 29524, 88573, 265720, 797161, 2391484, 7174453,
];

impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        println!("{} of:", n);
        fn recursive_check_powers_of_three(n: i32, exclude: i32) -> bool {
            for x in POWERS_OF_THREE {
                if x == n {
                    return x != exclude;
                }
                if x > n {
                    break;
                }
            }
            for x in FULL_SUM_POWERS_OF_THREE {
                if x == n {
                    return x != exclude;
                }
                if x > n {
                    break;
                }
            }
            for i in 1..MEM_SIZE {
                if FULL_SUM_POWERS_OF_THREE[i - 1] < n && n < POWERS_OF_THREE[i] {
                    return false;
                }
            }

            for i in 0..MEM_SIZE {
                let m = POWERS_OF_THREE[MEM_SIZE - i - 1];
                if m < exclude {
                    let new_n = n - m;
                    if new_n > 0 && recursive_check_powers_of_three(new_n, m) {
                        return true;
                    }
                }
            }

            return false;
        }
        return recursive_check_powers_of_three(n, 4782969 + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn problem_1780_ex1() {
        assert_eq!(Solution::check_powers_of_three(12), true);
    }

    #[test]
    fn problem_1780_ex2() {
        assert_eq!(Solution::check_powers_of_three(91), true);
    }

    #[test]
    fn problem_1780_ex3() {
        assert_eq!(Solution::check_powers_of_three(21), false);
    }

    #[test]
    fn problem_1780_ex4() {
        assert_eq!(Solution::check_powers_of_three(7174452), true);
    }

    #[test]
    fn problem_1780_ex5() {
        assert_eq!(Solution::check_powers_of_three(7174449), true);
    }

    #[test]
    fn problem_1780_ex6() {
        assert_eq!(Solution::check_powers_of_three(7174450), true);
    }

    #[test]
    fn problem_1780_ex7() {
        assert_eq!(Solution::check_powers_of_three(11), false);
    }
    #[test]
    fn problem_1780_ex8() {
        assert_eq!(Solution::check_powers_of_three(9565938), false);
    }
}
