use std::slice::Iter;

pub struct Solution;

const SEQUENCE_MAX: i32 = 1000_i32;
const SEQUENCE_MIN: i32 = 0;
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        // for _ in k {
        //     let i = prices.iter();
        //     Solution::max_profit_in_sub(i);
        // }
        return 0;
    }

    // pub fn max_profit_in_sub(prices: Iter<i32>) -> i32 {
    //     let mut mem_max_price = vec![0; prices.len()];
    //     let mut max = prices.last().unwrap().clone();
    //
    //     for i in prices.len()..0 {
    //         if prices[i] > max {
    //             max = prices[i]
    //         }
    //         mem_max_price[i] = max
    //     }
    //
    //     return 0;
    // }
    //
    // pub fn max_profit_for_1(prices: &Vec<i32>) -> i32 {
    //     let n = prices.len();
    //     let mut mem_max_price = vec![0; n];
    //     let mut max_price = prices[n - 1];
    //
    //     for i in (0..n).rev() {
    //         if prices[i] > max_price {
    //             max_price = prices[i];
    //         }
    //         mem_max_price[i] = max_price;
    //     }
    //
    //     let mut profit = 0;
    //     for i in 0..n {
    //         let cur_profit = mem_max_price[i] - prices[i];
    //         if cur_profit > profit {
    //             profit = cur_profit;
    //         }
    //     }
    //
    //     return profit;
    // }
    //
    // pub fn max_profit1(k: i32, prices: Vec<i32>) -> i32 {
    //     let n = prices.len();
    //     if n == 0 {
    //         return 0;
    //     }
    //
    //     if k == 1 {
    //         return Solution::max_profit_for_1(&prices);
    //     }
    //
    //     if k as usize >= n / 2 {
    //         let mut profit = 0;
    //         for i in 1..n {
    //             if prices[i] > prices[i - 1] {
    //                 profit += prices[i] - prices[i - 1];
    //             }
    //         }
    //         return profit;
    //     }
    //
    //
    //     let mut deduplicated_prices = Vec::new();
    //
    //     deduplicated_prices.push((0, prices[0]));
    //
    //     for (index, price) in prices.iter().enumerate() {
    //         if let Some((_, v)) = deduplicated_prices.last() {
    //             if v != price {
    //                 deduplicated_prices.push((index, *price))
    //             }
    //         }
    //     }
    //
    //     let prices_count = deduplicated_prices.len();
    //
    //     let mut local_max = deduplicated_prices.first().unwrap().clone();
    //     let mut local_min = deduplicated_prices.first().unwrap().clone();
    //
    //     let mut local_maxs = Vec::new();
    //     let mut local_mins = Vec::new();
    //
    //     for i in 1..deduplicated_prices.len() {
    //         let (prev, cur) = (deduplicated_prices[i - 1], deduplicated_prices[i]);
    //
    //         if local_max.1 < cur.1 {
    //             local_max = cur;
    //         }
    //         if local_min.1 > cur.1 {
    //             local_min = cur;
    //         }
    //
    //         if i + 1 < prices_count {
    //             let next = deduplicated_prices[i + 1];
    //             // нашли локальный максиум нам нужно сохранить локальный минимум и заново его иницировать
    //             if (prev.1 < cur.1 && cur.1 > next.1) {
    //                 local_mins.push(local_min);
    //                 local_min = cur;
    //             }
    //             // нашли локальный максиум нам нужно сохранить локальный минимум и заново его иницировать
    //             if (prev.1 > cur.1 && cur.1 < next.1) {
    //                 local_maxs.push(local_max);
    //                 local_max = cur;
    //             }
    //         }
    //     }
    //
    //     local_mins.push(local_min);
    //     local_maxs.push(local_max);
    //
    //     if local_max[0]
    //
    //
    //     // let mut graph = vec![Vec::new(); 0];
    //
    //     println!("origin_prices: {:?}", prices);
    //     println!("deduped_prices: {:?}", deduplicated_prices);
    //     println!("local_maxs: {:?}", local_maxs);
    //     println!("local_mins: {:?}", local_mins);
    //
    //     return 0;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_188_ex1() {
        assert_eq!(Solution::max_profit(2, vec![2, 4, 1]), 2);
    }

    #[test]
    fn problem_188_ex2() {
        assert_eq!(Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
    }

    #[test]
    fn problem_188_ex3() {
        assert_eq!(Solution::max_profit(1, vec![3, 2, 6, 5, 0, 3]), 4);
    }

    #[test]
    fn problem_188_ex4() {
        assert_eq!(Solution::max_profit(1, vec![1, 2, 4, 2, 5, 6]), 5);
    }

    #[test]
    fn problem_188_ex5() {
        assert_eq!(Solution::max_profit(2, vec![1, 2, 4, 2, 5, 6]), 3 + 4);
    }

    #[test]
    fn problem_188_ex6() {
        assert_eq!(Solution::max_profit(2, vec![2, 4, 4, 1]), 2);
    }
}
