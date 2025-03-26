use std::ffi::c_ushort;

pub struct Solution;

const ANSWER_MOD: u64 = 1_000_000_007;

const TRANSITION_1: [&'static [u8]; 3] = [&[1, 2], &[0, 2], &[0, 1]];

const TRANSITION_2: [&'static [u8]; 6] = [
    &[2, 3, 4],
    &[2, 4, 5],
    &[0, 1, 5],
    &[0, 4, 5],
    &[0, 1, 3],
    &[1, 2, 3],
];

const TRANSITION_3: [&'static [u8]; 12] = [
    &[4, 5, 7, 8, 9],
    &[4, 6, 7, 8],
    &[4, 5, 8, 9, 11],
    &[5, 9, 10, 11],
    &[0, 1, 2, 10, 11],
    &[0, 2, 3, 10],
    &[1, 8, 9, 11],
    &[0, 1, 9, 10, 11],
    &[0, 1, 2, 6],
    &[0, 2, 3, 6, 7],
    &[3, 4, 5, 7],
    &[2, 3, 4, 6, 7],
];

const TRANSITION_4: [&'static [u8]; 24] = [
    &[8, 9, 10, 14, 15, 16, 17, 18],
    &[8, 10, 11, 14, 16, 18, 19],
    &[9, 12, 13, 15, 17],
    &[8, 9, 13, 14, 15, 16, 17],
    &[8, 9, 10, 16, 17, 18, 22],
    &[8, 10, 11, 16, 18, 19, 22, 23],
    &[11, 19, 20, 21, 23],
    &[10, 11, 18, 19, 20, 22, 23],
    &[0, 1, 3, 4, 5, 20, 21, 23],
    &[0, 2, 3, 4, 20, 22, 23],
    &[0, 1, 4, 5, 7, 20, 21],
    &[1, 5, 6, 7, 21],
    &[2, 16, 17, 18, 22],
    &[2, 3, 16, 18, 19, 22, 23],
    &[0, 1, 3, 19, 20, 21, 23],
    &[0, 2, 3, 18, 19, 20, 22, 23],
    &[0, 1, 3, 4, 5, 12, 13],
    &[0, 2, 3, 4, 12],
    &[0, 1, 4, 5, 7, 12, 13, 15],
    &[1, 5, 6, 7, 13, 14, 15],
    &[6, 7, 8, 9, 10, 14, 15],
    &[6, 8, 10, 11, 14],
    &[4, 5, 7, 9, 12, 13, 15],
    &[5, 6, 7, 8, 9, 13, 14, 15],
];

const TRANSITION_5: [&'static [u8]; 48] = [
    &[16, 17, 19, 20, 21, 28, 29, 31, 32, 33, 35, 36, 37],
    &[16, 18, 19, 20, 28, 30, 31, 32, 34, 35, 36],
    &[16, 17, 20, 21, 23, 28, 29, 32, 33, 36, 37, 39],
    &[17, 21, 22, 23, 29, 33, 37, 38, 39],
    &[18, 24, 25, 26, 30, 34],
    &[18, 19, 24, 26, 27, 30, 31, 34, 35],
    &[16, 17, 19, 27, 28, 29, 31, 32, 33, 35],
    &[16, 18, 19, 26, 27, 28, 30, 31, 32, 34, 35],
    &[16, 17, 19, 20, 21, 32, 33, 35, 36, 37, 44, 45],
    &[16, 18, 19, 20, 32, 34, 35, 36, 44],
    &[16, 17, 20, 21, 23, 32, 33, 36, 37, 39, 44, 45, 47],
    &[17, 21, 22, 23, 33, 37, 38, 39, 45, 46, 47],
    &[22, 23, 38, 39, 40, 41, 42, 46, 47],
    &[22, 38, 40, 42, 43, 46],
    &[20, 21, 23, 36, 37, 39, 41, 44, 45, 47],
    &[21, 22, 23, 37, 38, 39, 40, 41, 45, 46, 47],
    &[0, 1, 2, 6, 7, 8, 9, 10, 40, 41, 42, 46, 47],
    &[0, 2, 3, 6, 8, 10, 11, 40, 42, 43, 46],
    &[1, 4, 5, 7, 9, 41, 44, 45, 47],
    &[0, 1, 5, 6, 7, 8, 9, 40, 41, 45, 46, 47],
    &[0, 1, 2, 8, 9, 10, 14, 40, 41, 42],
    &[0, 2, 3, 8, 10, 11, 14, 15, 40, 42, 43],
    &[3, 11, 12, 13, 15, 43],
    &[2, 3, 10, 11, 12, 14, 15, 42, 43],
    &[4, 5, 32, 33, 35, 36, 37, 44, 45],
    &[4, 32, 34, 35, 36, 44],
    &[4, 5, 7, 32, 33, 36, 37, 39, 44, 45, 47],
    &[5, 6, 7, 33, 37, 38, 39, 45, 46, 47],
    &[0, 1, 2, 6, 7, 38, 39, 40, 41, 42, 46, 47],
    &[0, 2, 3, 6, 38, 40, 42, 43, 46],
    &[1, 4, 5, 7, 36, 37, 39, 41, 44, 45, 47],
    &[0, 1, 5, 6, 7, 37, 38, 39, 40, 41, 45, 46, 47],
    &[0, 1, 2, 6, 7, 8, 9, 10, 24, 25, 26],
    &[0, 2, 3, 6, 8, 10, 11, 24, 26, 27],
    &[1, 4, 5, 7, 9, 25],
    &[0, 1, 5, 6, 7, 8, 9, 24, 25],
    &[0, 1, 2, 8, 9, 10, 14, 24, 25, 26, 30],
    &[0, 2, 3, 8, 10, 11, 14, 15, 24, 26, 27, 30, 31],
    &[3, 11, 12, 13, 15, 27, 28, 29, 31],
    &[2, 3, 10, 11, 12, 14, 15, 26, 27, 28, 30, 31],
    &[12, 13, 15, 16, 17, 19, 20, 21, 28, 29, 31],
    &[12, 14, 15, 16, 18, 19, 20, 28, 30, 31],
    &[12, 13, 16, 17, 20, 21, 23, 28, 29],
    &[13, 17, 21, 22, 23, 29],
    &[8, 9, 10, 14, 18, 24, 25, 26, 30],
    &[8, 10, 11, 14, 15, 18, 19, 24, 26, 27, 30, 31],
    &[11, 12, 13, 15, 16, 17, 19, 27, 28, 29, 31],
    &[10, 11, 12, 14, 15, 16, 18, 19, 26, 27, 28, 30, 31],
];

impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        let transitions: &'static [&'static [u8]] = match m {
            1 => &TRANSITION_1[0..TRANSITION_1.len()],
            2 => &TRANSITION_2[0..TRANSITION_2.len()],
            3 => &TRANSITION_3[0..TRANSITION_3.len()],
            4 => &TRANSITION_4[0..TRANSITION_4.len()],
            _ => &TRANSITION_5[0..TRANSITION_5.len()],
        };

        let len = transitions.len();

        let mut dp = vec![1u64; len];
        for _ in 1..n {
            let mut dp_next = vec![0u64; len];
            for i in 0..len {
                for &j in transitions[i] {
                    dp_next[j as usize] = (dp_next[j as usize] + dp[i]) % ANSWER_MOD;
                }
            }
            dp = dp_next;
        }

        let res = dp.into_iter().fold(0u64, |acc, x| (acc + x) % ANSWER_MOD);
        res as i32
    }
}

fn compute_color_transition_options_of(m: usize) -> Vec<Vec<usize>> {
    let mut valid_columns = Vec::new();
    let total = 3usize.pow(m as u32);
    for num in 0..total {
        let mut config = Vec::with_capacity(m);
        let mut tmp = num;
        let mut valid = true;
        for _ in 0..m {
            config.push(tmp % 3);
            tmp /= 3;
        }
        // Проверяем, что соседние ячейки имеют разные цвета
        for i in 1..m {
            if config[i] == config[i - 1] {
                valid = false;
                break;
            }
        }
        if valid {
            valid_columns.push(config);
        }
    }

    // Построим переходы между столбцами: два соседних столбца допустимы, если во всех позициях цвета различны
    let len = valid_columns.len();
    let mut transitions = vec![Vec::new(); len];
    for i in 0..len {
        for j in 0..len {
            let mut ok = true;
            for k in 0..m {
                if valid_columns[i][k] == valid_columns[j][k] {
                    ok = false;
                    break;
                }
            }
            if ok {
                transitions[i].push(j);
            }
        }
    }
    return transitions;
}

fn compute_all_transition() {
    for i in 1..6 {
        let tran = compute_color_transition_options_of(i);
        println!("const TRANSITION_{}: [&'static [u8]; {}] = [", i, tran.len());
        for transition in tran.iter() {
            println!("    &{:?},", transition,);
        }
        println!("];");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_1931_ex1() {
        assert_eq!(Solution::color_the_grid(1, 1,), 3);
    }

    #[test]
    fn problem_1931_ex2() {
        assert_eq!(Solution::color_the_grid(1, 2), 6);
    }

    #[test]
    fn problem_1931_ex3() {
        assert_eq!(Solution::color_the_grid(5, 5), 580986);
    }

    #[test]
    fn problem_1931_ex4() {
        assert_eq!(Solution::color_the_grid(2, 1), 6);
    }

    #[test]
    fn problem_1931_ex5() {
        assert_eq!(Solution::color_the_grid(2, 2), 18);
    }

    #[test]
    fn problem_1931_ex6() {
        assert_eq!(Solution::color_the_grid(2, 5), 486);
    }

    #[test]
    fn problem_1931_ex7() {
        assert_eq!(Solution::color_the_grid(1, 3), 12);
    }

    #[test]
    fn problem_1931_ex8() {
        assert_eq!(Solution::color_the_grid(1, 4), 24);
    }

    #[test]
    fn problem_1931_ex9() {
        assert_eq!(Solution::color_the_grid(5, 1), 48);
    }
}
