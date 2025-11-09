use std::collections::HashSet;
pub struct Solution;
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut set = HashSet::new();
        let mut current = n;
        loop {
            if current == 1 {
                return true;
            }
            if set.contains(&current) {
                return false;
            }
            set.insert(current);
            current = Solution::sum_of_sq_digits(current);
        }
    }

    pub fn sum_of_sq_digits(n: i32) -> i32 {
        let mut n = n;
        let mut sum: i32 = 0;

        loop {
            let m = n / 10;
            sum += (n % 10) * (n % 10);
            if m == 0 {
                break;
            }
            n = m;
        }
        sum
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let set = HashSet::from([
            1, 7, 10, 13, 19, 23, 28, 31, 32, 44, 49, 68, 70, 79, 82, 86, 91, 94, 97, 100,
        ]);

        for i in 1..=100 {
            if set.contains(&i) {
                assert_eq!(
                    Solution::is_happy(i),
                    true,
                    "should be happy, failed at {}",
                    i
                );
            } else {
                assert_eq!(
                    Solution::is_happy(i),
                    false,
                    "shouldn't be happy, failed at {}",
                    i
                );
            }
        }
    }
}
