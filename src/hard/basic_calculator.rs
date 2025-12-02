pub struct Solution;
// LeetCode Hard 224. Basic Calculator
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut nums: Vec<i32> = vec![];
        let mut ops: Vec<char> = vec![];

        let mut num: Vec<char> = vec![];

        for i in 0..s.len() {
            let c = s[i];
            match c {
                ' ' => continue,
                '0'..='9' => {
                    num.push(c);
                    if i == s.len() - 1 || !(s[i + 1] >= '0' && s[i + 1] <= '9') {
                        nums.push((num.iter().collect::<String>()).parse::<i32>().unwrap());
                        num.clear();
                        if ops.len() == 0 || *ops.last().unwrap() == '(' {
                            ops.push('+');
                        }
                    }
                }
                '(' | '+' | '-' => {
                    ops.push(c);
                }
                ')' => {
                    //println!("{:?} {:?}", n, o);
                    let mut tmp = 0;
                    while *ops.last().unwrap() != '(' {
                        tmp += Solution::get_signed_value(nums.pop().unwrap(), ops.pop().unwrap());
                    }
                    ops.pop(); //pop '('
                    nums.push(tmp);
                    if ops.len() == 0 || *ops.last().unwrap() == '(' {
                        ops.push('+');
                    }
                }
                _ => continue,
            }
        }

        println!("{:?} {:?}", nums, ops);

        let mut res = 0;
        if ops.len() == 0 {
            if num.len() > 0 {
                res = (num.iter().collect::<String>()).parse::<i32>().unwrap();
            } else {
                res = nums.pop().unwrap();
            }
        } else {
            while ops.len() > 0 {
                res += Solution::get_signed_value(nums.pop().unwrap(), ops.pop().unwrap());
            }
        }
        return res;
    }

    fn get_signed_value(a: i32, o: char) -> i32 {
        match o {
            '+' => a,
            '-' => -a,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let s = "1234".to_string();
        let result = Solution::calculate(s);
        assert_eq!(1234, result);
    }

    #[test]
    fn test_1() {
        let s = "1 + 1".to_string();
        let result = Solution::calculate(s);
        assert_eq!(2, result);
    }

    #[test]
    fn test_2() {
        let s = " 2-1 + 2 ".to_string();
        let result = Solution::calculate(s);
        assert_eq!(3, result);
    }

    #[test]
    fn test_3() {
        let s = "(1+(4+5+2)-3)+(6+8)".to_string();
        let result = Solution::calculate(s);
        assert_eq!(23, result);
    }

    #[test]
    fn test_4() {
        let s = "- (3 + (4 + 5))".to_string();
        let result = Solution::calculate(s);
        assert_eq!(-12, result);
    }

    #[test]
    fn test_5() {
        let s = "1- (-2)".to_string();
        let result = Solution::calculate(s);
        assert_eq!(3, result);
    }

    #[test]
    fn test_6() {
        let s = "-1- (-2)".to_string();
        let result = Solution::calculate(s);
        assert_eq!(1, result);
    }

    #[test]
    fn test_7() {
        let s = "1-(1-(2-(3-1)))".to_string();
        let result = Solution::calculate(s);
        assert_eq!(0, result);
    }

    #[test]
    fn test_8() {
        let s = "-(12-4)".to_string();
        let result = Solution::calculate(s);
        assert_eq!(-8, result);
    }
}
