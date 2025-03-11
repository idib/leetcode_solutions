pub struct Solution;

impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let mut chars = s.chars();
        // Извлекаем начальные координаты
        let Some(col1) = chars.next() else {
            return vec![];
        };
        let Some(row1) = chars.next() else {
            return vec![];
        };
        chars.next(); // Пропускаем символ ':'
        // Извлекаем конечные координаты
        let Some(col2) = chars.next() else {
            return vec![];
        };
        let Some(row2) = chars.next() else {
            return vec![];
        };

        let n = (col2 as u8 - col1 as u8) as usize + 1;
        let m = (row2 as u8 - row1 as u8) as usize + 1;

        let mut res = Vec::with_capacity(n * m);

        for x in 0..n {
            for y in 0..m {
                let cell = format!(
                    "{}{}",
                    (col1 as u8 + x as u8) as char,
                    (row1 as u8 + y as u8) as char
                );
                res.push(cell);
            }
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn problem_2194_ex1() {
        assert_eq!(
            Solution::cells_in_range(String::from("K1:L2")),
            vec!["K1", "K2", "L1", "L2"]
        );
    }
    #[test]
    fn problem_2194_ex2() {
        assert_eq!(
            Solution::cells_in_range(String::from("A1:F1")),
            vec!["A1", "B1", "C1", "D1", "E1", "F1"]
        );
    }
}
