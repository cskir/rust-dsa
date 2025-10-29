pub struct Solution {}

impl Solution {
    fn calc_min_step(height: &Vec<i32>, start: usize, end: usize) -> Option<i32> {
        let mut min_value: Option<i32> = None;

        for i in start..=end {
            if height[i] > 0 {
                match min_value {
                    Some(min_value) if height[i] >= min_value => {}
                    _ => min_value = Some(height[i]),
                }
                {}
            }
        }

        min_value
    }

    pub fn trap(height: Vec<i32>) -> i32 {
        // let mut min_value: Option<i32> = None;

        // for &i in &height {
        //     if i > 0 {
        //         match min_value {
        //             Some(min_value) if i >= min_value => {}
        //             _ => min_value = Some(i),
        //         }
        //         {}
        //     }
        // }

        // if min_value.is_none() {
        //     return 0;
        // }

        // let mut min_value = min_value.unwrap();

        let mut min_value: Option<i32>;
        let mut res = 0;
        let mut height = height;

        let mut start = 0;
        let mut end = height.len() - 1;

        while end - start > 1 {
            println!("{:?}", height);

            let mut index_start_nz: Option<usize> = None;

            let mut index_end_nz: Option<usize> = None;

            for i in start..=end {
                if height[i] != 0 {
                    index_start_nz = Some(i);
                    break;
                }
            }

            for i in (start..=end).rev() {
                if height[i] != 0 {
                    index_end_nz = Some(i);
                    break;
                }
            }

            if index_start_nz.is_none() {
                break;
            }

            start = match index_start_nz {
                None => start,
                Some(x) => x,
            };

            end = match index_end_nz {
                None => end,
                Some(x) => x,
            };

            println!("start: {} end: {}", start, end);

            min_value = Self::calc_min_step(&height, start, end);

            if min_value.is_none() {
                break;
            }

            println!("min value: {:?}", min_value);

            for i in start..=end {
                if height[i] == 0 {
                    res += min_value.unwrap();
                } else {
                    height[i] -= min_value.unwrap();
                }
            }

            println!("res: {} ", res);
        }

        res as i32
    }

    pub fn trap2(height: Vec<i32>) -> i32 {
        let mut min_value: Option<i32> = None;

        for &i in &height {
            if i > 0 {
                match min_value {
                    Some(min_value) if i >= min_value => {}
                    _ => min_value = Some(i),
                }
                {}
            }
        }

        if min_value.is_none() {
            return 0;
        }

        let mut min_value = min_value.unwrap();
        let mut res = 0;

        let mut height = height;
        let mut next_level_indices: Vec<usize> = (0..height.len()).collect();

        while next_level_indices.len() > 1 {
            println!("{:?}", height);
            let mut tmp: Vec<usize> = Vec::new();
            let mut prev: Option<usize> = None;

            for i in next_level_indices.iter() {
                if height[*i] > 0 {
                    if prev.is_some() {
                        res += (*i - 1 - prev.unwrap()) * min_value as usize;
                    }
                    prev = Some(*i);

                    if height[*i] > min_value {
                        height[*i] -= min_value;
                        tmp.push(*i);
                    }
                }
            }
            next_level_indices = tmp;
            min_value = 1;
        }

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let height = vec![0];
        assert_eq!(0, Solution::trap(height));
    }

    #[test]
    fn test_0_0() {
        let height = vec![0, 0];
        assert_eq!(0, Solution::trap(height));
    }

    #[test]
    fn test_1_0() {
        let height = vec![1, 0];
        assert_eq!(0, Solution::trap(height));
    }

    #[test]
    fn test_1_1() {
        let height = vec![1, 1];
        assert_eq!(0, Solution::trap(height));
    }

    #[test]
    fn test_0_1_0() {
        let height = vec![0, 1, 0];
        assert_eq!(0, Solution::trap(height));
    }

    #[test]
    fn test_0_1_2() {
        let height = vec![0, 1, 2];
        assert_eq!(0, Solution::trap(height));
    }

    #[test]
    fn test_2_1_0() {
        let height = vec![2, 1, 0];
        assert_eq!(0, Solution::trap(height));
    }

    #[test]
    fn test_1_0_1() {
        let height = vec![1, 0, 1];
        assert_eq!(1, Solution::trap(height));
    }

    #[test]
    fn test_1_0_1_0_1() {
        let height = vec![1, 0, 1, 0, 1];
        assert_eq!(2, Solution::trap(height));
    }

    #[test]
    fn test_2_0_2_0_2() {
        let height = vec![2, 0, 2, 0, 2];
        assert_eq!(4, Solution::trap(height));
    }

    #[test]
    fn test_2_0_3_0_2() {
        let height = vec![2, 0, 3, 0, 2];
        assert_eq!(4, Solution::trap(height));
    }

    #[test]
    fn test_2_0_1_0_2() {
        let height = vec![2, 0, 1, 0, 2];
        assert_eq!(5, Solution::trap(height));
    }

    #[test]
    fn test_2_0_0_0_2() {
        let height = vec![2, 0, 0, 0, 2];
        assert_eq!(6, Solution::trap(height));
    }

    #[test]
    fn test_0_1_0_2_1_0_1_3_2_1_2_1() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(6, Solution::trap(height));
    }

    #[test]
    fn test_4_2_0_3_2_5() {
        let height = vec![4, 2, 0, 3, 2, 5];
        assert_eq!(9, Solution::trap(height));
    }

    #[test]
    fn test_11_0_10_0_9_0_8_0_7_0_6() {
        let height = vec![11, 0, 10, 0, 9, 0, 8, 0, 7, 0, 6];
        assert_eq!(40, Solution::trap(height));
    }

    #[test]
    fn test_11_5_10_5_9_5_8_5_7_5_6() {
        let height = vec![11, 5, 10, 5, 9, 5, 8, 5, 7, 5, 6];
        assert_eq!(15, Solution::trap(height));
    }

    #[test]
    fn test_11_6_10_6_9_6_8_6_7_6_6() {
        let height = vec![11, 6, 10, 6, 9, 6, 8, 6, 7, 6, 6];
        assert_eq!(10, Solution::trap(height));
    }

    #[test]
    fn test_11_0_0_10_0_0_9_0_0_8_0_0_7_0_0_6() {
        let height = vec![11, 0, 0, 10, 0, 0, 9, 0, 0, 8, 0, 0, 7, 0, 0, 6];
        assert_eq!(80, Solution::trap(height));
    }

    #[test]
    fn test_11_0_3_10_0_3_9_0_3_8_0_3_7_0_3_6() {
        let height = vec![11, 0, 3, 10, 0, 3, 9, 0, 3, 8, 0, 3, 7, 0, 3, 6];
        assert_eq!(65, Solution::trap(height));
    }
}
