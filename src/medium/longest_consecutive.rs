use std::collections::HashSet;
pub struct Solution;
impl Solution {
    // vec based on set to avoid set iteration overhead
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = HashSet::new();
        let mut items: Vec<i32> = Vec::new();
        for num in &nums {
            if !set.contains(num) {
                set.insert(*num);
                items.push(*num);
            }
        }

        let mut max = 0;
        loop {
            if let Some(num) = items.pop() {
                let mut local = 1;

                set.remove(&num);
                let mut step = 1;
                while set.contains(&(num - step)) {
                    set.remove(&(num - step));
                    local += 1;
                    step += 1;
                }
                step = 1;
                while set.contains(&(num + step)) {
                    set.remove(&(num + step));
                    local += 1;
                    step += 1;
                }

                if local > max {
                    max = local;
                }
            } else {
                break;
            }

            if set.is_empty() {
                break;
            }
        }

        max
    }

    // this has worse performance due to the interator overhead
    pub fn longest_consecutive_set_only(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = HashSet::new();

        for num in &nums {
            set.insert(*num);
        }

        let mut max = 0;
        while set.len() > 0 {
            let num = *set.iter().next().unwrap();
            let mut local = 1;

            set.remove(&num);

            let mut can_continue = true;
            let mut step_down = 1;
            let mut step_up = 1;

            while can_continue {
                can_continue = false;
                if set.remove(&(num - step_down)) {
                    local += 1;
                    step_down += 1;
                    can_continue = true;
                }
                if set.remove(&(num + step_up)) {
                    local += 1;
                    step_up += 1;
                    can_continue = true;
                }
            }

            if local > max {
                max = local;
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        let result = Solution::longest_consecutive(nums);
        assert_eq!(4, result);
    }

    #[test]
    fn test_2() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        let result = Solution::longest_consecutive(nums);
        assert_eq!(9, result);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 0, 1, 2];
        let result = Solution::longest_consecutive(nums);
        assert_eq!(3, result);
    }

    #[test]
    fn test_4() {
        let nums = vec![1, 2, 2, 2, 3];
        let result = Solution::longest_consecutive(nums);
        assert_eq!(3, result);
    }

    #[test]
    fn test_5() {
        let nums = vec![0, 0, 1, 2, 3, 4, 5, 6, 7, 8];
        let result = Solution::longest_consecutive(nums);
        assert_eq!(9, result);
    }
}
