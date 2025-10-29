pub struct Solution {}

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut res: Vec<String> = vec![];

        // guaranteed by the constraints
        let mut line_len = max_width - words[0].len() as i32;
        let mut line_idx: Vec<usize> = vec![0];

        for i in 1..words.len() {
            if line_len - words[i].len() as i32 - line_idx.len() as i32 >= 0 {
                line_idx.push(i);
                line_len -= words[i].len() as i32;
            } else {
                let mut s = words[line_idx[0]].clone();
                if line_idx.len() == 1 {
                    let space = max_width - words[line_idx[0]].len() as i32;
                    s.push_str((0..space).map(|_| " ").collect::<String>().as_str());
                } else {
                    let even_space = line_len / (line_idx.len() as i32 - 1);
                    let remain_space = line_len % (line_idx.len() as i32 - 1);

                    for j in 1..line_idx.len() {
                        if remain_space - j as i32 >= 0 {
                            s.push(' ');
                        }

                        s.push_str((0..even_space).map(|_| " ").collect::<String>().as_str());
                        s.push_str(words[line_idx[j]].as_str());
                    }
                }
                res.push(s);

                line_idx = vec![i];
                line_len = max_width - words[i].len() as i32;
            }
        }

        if line_idx.len() > 0 {
            let mut s = words[line_idx[0]].clone();
            for j in 1..line_idx.len() {
                s.push_str(" ");
                s.push_str(words[line_idx[j]].as_str());
            }

            let space = max_width - s.len() as i32;
            if space > 0 {
                s.push_str((0..space).map(|_| " ").collect::<String>().as_str());
            }

            res.push(s);
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn convert_from_str(strs: Vec<&'static str>) -> Vec<String> {
        strs.iter().map(|i| i.to_string()).collect()
    }

    fn convert_to_str<'a>(strs: &'a [String]) -> Vec<&'a str> {
        strs.iter().map(|i| i.as_str()).collect()
    }

    #[test]
    fn test_1() {
        let words = vec![
            "This",
            "is",
            "an",
            "example",
            "of",
            "text",
            "justification.",
        ];
        let expected = vec!["This    is    an", "example  of text", "justification.  "];
        let actual = Solution::full_justify(convert_from_str(words), 16);
        assert_eq!(expected, convert_to_str(&actual));
    }

    #[test]
    fn test_2() {
        let words = vec!["What", "must", "be", "acknowledgment", "shall", "be"];
        let expected = vec!["What   must   be", "acknowledgment  ", "shall be        "];
        let actual = Solution::full_justify(convert_from_str(words), 16);
        assert_eq!(expected, convert_to_str(&actual));
    }

    #[test]
    fn test_3() {
        let words = vec![
            "Science",
            "is",
            "what",
            "we",
            "understand",
            "well",
            "enough",
            "to",
            "explain",
            "to",
            "a",
            "computer.",
            "Art",
            "is",
            "everything",
            "else",
            "we",
            "do",
        ];
        let expected = vec![
            "Science  is  what we",
            "understand      well",
            "enough to explain to",
            "a  computer.  Art is",
            "everything  else  we",
            "do                  ",
        ];
        let actual = Solution::full_justify(convert_from_str(words), 20);
        assert_eq!(expected, convert_to_str(&actual));
    }
}
