pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        // height.len() > 1
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut area;
        let mut max_area = 0;

        while left < right {
            area = std::cmp::min(height[left], height[right]) * (right - left) as i32;

            if area > max_area {
                max_area = area;
            }

            if height[left] < height[right] {
                left += 1;
            } else if height[left] > height[right] {
                right -= 1;
            } else if height[left + 1] < height[right - 1] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let height = vec![0, 0];
        assert_eq!(0, Solution::max_area(height));
    }

    #[test]
    fn test_2() {
        let height = vec![0, 1];
        assert_eq!(0, Solution::max_area(height));
    }

    #[test]
    fn test_3() {
        let height = vec![1, 1];
        assert_eq!(1, Solution::max_area(height));
    }

    #[test]
    fn test_4() {
        let height = vec![2, 1];
        assert_eq!(1, Solution::max_area(height));
    }

    #[test]
    fn test_5() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(49, Solution::max_area(height));
    }

    #[test]
    fn test_6() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(49, Solution::max_area(height));
    }
}
