pub struct Solution {}
// LeetCode Medium 150. Evaluate Reverse Polish Notation
impl Solution {
    pub fn eval_reverse_polish_notation(tokens: Vec<String>) -> i32 {
        //Constraint: 1 <= tokens.length
        let mut stack: Vec<i32> = vec![];
        for x in tokens {
            match x.as_str() {
                "+" | "-" | "*" | "/" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let tmp = match x.as_str() {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => a / b,
                        _ => unreachable!(),
                    };
                    stack.push(tmp);
                }

                x => {
                    let y = x.parse::<i32>().unwrap();
                    stack.push(y);
                }
            }
        }
        stack.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let tokens = vec!["2", "1", "+", "3", "*"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let result = Solution::eval_reverse_polish_notation(tokens);
        assert_eq!(9, result);
    }

    #[test]
    fn test_2() {
        let tokens = vec!["4", "13", "5", "/", "+"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let result = Solution::eval_reverse_polish_notation(tokens);
        assert_eq!(6, result);
    }

    #[test]
    fn test_3() {
        let tokens = vec![
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let result = Solution::eval_reverse_polish_notation(tokens);
        assert_eq!(22, result);
    }
}
