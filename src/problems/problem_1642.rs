pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
        let mut upper_delta_floors: BinaryHeap<i32> = BinaryHeap::with_capacity(heights.len());

        let mut prev_height = heights[0];
        let n = heights.len();

        for i in 1..n {
            let delta = heights[i] - prev_height;
            if delta > 0 {
                upper_delta_floors.push(delta);
                bricks -= delta;

                if bricks < 0 {
                    if ladders == 0 {
                        return (i - 1) as i32;
                    }
                    ladders -= 1;
                    bricks += upper_delta_floors.pop().unwrap();
                }
            }
            prev_height = heights[i];
        }

        return (heights.len() - 1) as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_furthest_building_ex1() {
        assert_eq!(
            Solution::furthest_building(vec![4, 2, 7, 6, 9, 14, 12], 5, 1),
            4
        );
    }

    #[test]
    fn test_furthest_building_ex2() {
        assert_eq!(
            Solution::furthest_building(vec![4, 12, 2, 7, 3, 18, 20, 3, 19], 10, 2),
            7
        );
    }

    #[test]
    fn test_furthest_building_ex3() {
        assert_eq!(
            Solution::furthest_building(vec![14, 3, 19, 3], 17, 0),
            3 // хочу чтобы был перенос
        );
    }
}
