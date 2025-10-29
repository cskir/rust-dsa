pub struct Solution {}

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut res = ratings.len() as i32;
        let mut candies = vec![1; ratings.len()];
        let mut strict_inc_start: Option<usize> = None;
        let mut strict_dec_start: Option<usize> = None;

        for i in 0..ratings.len() {
            if strict_inc_start.is_some() {
                res += candies[i - 1] + 1 - candies[i];
                candies[i] = candies[i - 1] + 1;
            } else if strict_dec_start.is_some() {
                if candies[i - 1] <= candies[i] && candies[i] == 1 {
                    let mut min_value = 1 + (i - strict_dec_start.unwrap()) as i32;
                    for d in strict_dec_start.unwrap()..i {
                        if candies[d] < min_value {
                            res += min_value - candies[d];
                            candies[d] = min_value;
                        }
                        min_value -= 1;
                    }
                }
            }

            if i == ratings.len() - 1 {
                break;
            }

            if ratings[i] < ratings[i + 1] {
                if strict_inc_start.is_none() {
                    strict_inc_start = Some(i);
                    strict_dec_start = None;
                }
            } else if ratings[i] > ratings[i + 1] {
                if strict_dec_start.is_none() {
                    strict_dec_start = Some(i);
                    strict_inc_start = None;
                }
            } else {
                strict_inc_start = None;
                strict_dec_start = None;
            }
        }

        res
    }

    pub fn candy_2(ratings: Vec<i32>) -> i32 {
        let candies = Self::candies(ratings);
        let mut sum = 0;
        for i in 0..candies.len() {
            sum += candies[i];
        }
        sum
    }

    pub fn candies(ratings: Vec<i32>) -> Vec<i32> {
        println!("{:?}", ratings);
        let mut candies: Vec<i32> = vec![1; ratings.len()];

        let mut strict_inc_start: Option<usize> = None;
        let mut strict_dec_start: Option<usize> = None;

        // check last item separately
        for i in 0..ratings.len() {
            println!(
                "processing index: {} inc {:?} dec: {:?}",
                i, strict_inc_start, strict_dec_start
            );
            println!(" {:?}", candies);
            if strict_inc_start.is_some() {
                candies[i] = candies[i - 1] + 1;
            } else if strict_dec_start.is_some() {
                if candies[i - 1] <= candies[i] && candies[i] == 1 {
                    let mut min_value = 1 + (i - strict_dec_start.unwrap()) as i32;
                    for d in strict_dec_start.unwrap()..i {
                        if candies[d] < min_value {
                            candies[d] = min_value;
                        }
                        min_value -= 1;
                    }
                }
            }

            println!(" {:?}", candies);

            if i == ratings.len() - 1 {
                break;
            }

            if ratings[i] < ratings[i + 1] {
                if strict_inc_start.is_none() {
                    strict_inc_start = Some(i);
                    strict_dec_start = None;
                }
            } else if ratings[i] > ratings[i + 1] {
                if strict_dec_start.is_none() {
                    strict_dec_start = Some(i);
                    strict_inc_start = None;
                }
            } else {
                strict_inc_start = None;
                strict_dec_start = None;
            }
        }

        candies
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let ratings = vec![0];
        assert_eq!(vec![1], Solution::candies(ratings));
    }

    #[test]
    fn test_1() {
        let ratings = vec![1];
        assert_eq!(vec![1], Solution::candies(ratings));
    }

    #[test]
    fn test_1_2_3_1_0() {
        let ratings = vec![1, 2, 3, 1, 0];
        assert_eq!(vec![1, 2, 3, 2, 1], Solution::candies(ratings));
    }

    #[test]
    fn test_1_0_2() {
        let ratings = vec![1, 0, 2];
        assert_eq!(vec![2, 1, 2], Solution::candies(ratings));
    }

    #[test]
    fn test_1_2_2() {
        let ratings = vec![1, 2, 2];
        assert_eq!(vec![1, 2, 1], Solution::candies(ratings));
    }

    #[test]
    fn test_1_0_2_3_1_2() {
        let ratings = vec![1, 0, 2, 3, 1, 2];
        assert_eq!(vec![2, 1, 2, 3, 1, 2], Solution::candies(ratings));
    }

    #[test]
    fn test_1_0_1_1_1() {
        let ratings = vec![1, 0, 1, 1, 1];
        assert_eq!(vec![2, 1, 2, 1, 1], Solution::candies(ratings));
    }

    #[test]
    fn test_1_2_0_2_1_1_0_1() {
        let ratings = vec![1, 2, 0, 2, 1, 1, 0, 1];
        assert_eq!(vec![1, 2, 1, 2, 1, 2, 1, 2], Solution::candies(ratings));
    }

    #[test]
    fn test_1_3_0_3_2_1_0_1() {
        let ratings = vec![1, 3, 0, 3, 2, 1, 0, 1];
        assert_eq!(vec![1, 2, 1, 4, 3, 2, 1, 2], Solution::candies(ratings));
    }

    #[test]
    fn test_0_0_0_3_2_1_0_3() {
        let ratings = vec![0, 0, 0, 3, 2, 1, 0, 3];
        assert_eq!(vec![1, 1, 1, 4, 3, 2, 1, 2], Solution::candies(ratings));
    }
}
