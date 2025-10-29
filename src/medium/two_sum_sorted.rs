pub struct Solution {}
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut objective: i32;

        for i in 0..numbers.len() {
            objective = target - numbers[i];
            println!("i {} obj {}", i, objective);
            let mut dn = i;
            let mut up = numbers.len();
            let mut j: usize;
            loop {
                j = (up + dn) / 2;
                if objective < numbers[j] {
                    up = j;
                } else if numbers[j] < objective {
                    dn = j;
                } else {
                    return vec![i as i32 + 1, j as i32 + 1];
                }
                println!("dn {} up {}", dn, up);
                if dn + 1 == up {
                    break;
                }
            }
        }

        // constraint: exactly one solutino
        vec![0, 0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let numbers = vec![2, 7, 11, 15];
        assert_eq!(vec![1, 2], Solution::two_sum(numbers, 9));
    }

    #[test]
    fn test_2() {
        let numbers = vec![2, 3, 4];
        assert_eq!(vec![1, 3], Solution::two_sum(numbers, 6));
    }

    #[test]
    fn test_3() {
        let numbers = vec![-1, 0];
        assert_eq!(vec![1, 2], Solution::two_sum(numbers, -1));
    }

    #[test]
    fn test_4() {
        let numbers = vec![2, 7, 11, 15, 17, 22, 29];
        assert_eq!(vec![4, 5], Solution::two_sum(numbers, 32));
    }

    #[test]
    fn test_5() {
        let numbers = vec![2, 7, 11, 15, 17, 22, 29, 32, 32];
        assert_eq!(vec![8, 9], Solution::two_sum(numbers, 64));
    }
}
