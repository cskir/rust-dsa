// LeetCode Medium 71. Simplify Path
pub struct Solution {}
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut result = String::new();
        let s_path = path.split('/').collect::<Vec<&str>>();
        let mut stack: Vec<&str> = Vec::new();
        for i in s_path {
            if i == "" || i == "." {
                continue;
            } else if i == ".." {
                stack.pop();
            } else {
                stack.push(i);
            }
        }

        for i in &stack {
            result.push('/');
            result.push_str(i);
        }

        if result.len() == 0 {
            result.push('/');
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let input = String::from("//");
        let result = Solution::simplify_path(input);
        assert_eq!(result, String::from("/"));
    }

    #[test]
    fn test_1() {
        let input = String::from("/home/");
        let result = Solution::simplify_path(input);
        assert_eq!(result, String::from("/home"));
    }

    #[test]
    fn test_2() {
        let input = String::from("/../");
        let result = Solution::simplify_path(input);
        assert_eq!(result, String::from("/"));
    }

    #[test]
    fn test_3() {
        let input = String::from("/home//foo/");
        let result = Solution::simplify_path(input);
        assert_eq!(result, String::from("/home/foo"));
    }

    #[test]
    fn test_4() {
        let input = String::from("/a/./b/../../c/");
        let result = Solution::simplify_path(input);
        assert_eq!(result, String::from("/c"));
    }

    #[test]
    fn test_5() {
        let input = String::from("/a/../../b/../c//.//");
        let result = Solution::simplify_path(input);
        assert_eq!(result, String::from("/c"));
    }

    #[test]
    fn test_6() {
        let input = String::from("/a//b////c/d//././/..");
        let result = Solution::simplify_path(input);
        assert_eq!(result, String::from("/a/b/c"));
    }

    #[test]
    fn test_7() {
        let input = String::from("/.../a/../b/c/../d/./");
        let result = Solution::simplify_path(input);
        assert_eq!(result, String::from("/.../b/d"));
    }
}
