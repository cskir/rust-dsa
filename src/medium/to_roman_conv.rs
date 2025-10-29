pub struct Solution {}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut res = String::new();
        let ten: i32 = 10;
        let mut num = num;
        let mut value;
        for i in (0..=3).rev() {
            value = num / ten.pow(i);
            num -= value * ten.pow(i);

            if value == 4 {
                res.push_str(Self::solve_4(i));
            } else if value == 5 {
                res.push_str(Self::solve_5(i));
            } else if value == 9 {
                res.push_str(Self::solve_9(i));
            } else {
                if value > 5 {
                    res.push_str(Self::solve_5(i));
                    value -= 5;
                }
                res.push_str(Self::solve_1(i).repeat(value as usize).as_str());
            }
        }
        res
    }

    pub fn solve_1(pos: u32) -> &'static str {
        match pos {
            0 => "I",
            1 => "X",
            2 => "C",
            3 => "M",
            _ => "",
        }
    }

    pub fn solve_4(pos: u32) -> &'static str {
        match pos {
            0 => "IV",
            1 => "XL",
            2 => "CD",
            _ => "",
        }
    }

    pub fn solve_5(pos: u32) -> &'static str {
        match pos {
            0 => "V",
            1 => "L",
            2 => "D",
            _ => "",
        }
    }

    pub fn solve_9(pos: u32) -> &'static str {
        match pos {
            0 => "IX",
            1 => "XC",
            2 => "CM",
            _ => "String::new()",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::easy::from_roman_conv;

    #[test]
    fn test_all() {
        for i in 0..4000 {
            assert_eq!(
                i,
                from_roman_conv::Solution::roman_to_int(Solution::int_to_roman(i))
            );
        }
    }

    #[test]
    fn test_1() {
        assert_eq!("I".to_string(), Solution::int_to_roman(1));
        assert_eq!("II".to_string(), Solution::int_to_roman(2));
        assert_eq!("IV".to_string(), Solution::int_to_roman(4));
        assert_eq!("V".to_string(), Solution::int_to_roman(5));
        assert_eq!("VI".to_string(), Solution::int_to_roman(6));
        assert_eq!("VII".to_string(), Solution::int_to_roman(7));

        assert_eq!("MMDCCCXXX".to_string(), Solution::int_to_roman(2830));
        assert_eq!("DCLXXXII".to_string(), Solution::int_to_roman(682));
        assert_eq!("MMDCCLXXIX".to_string(), Solution::int_to_roman(2779));
        assert_eq!("DXXVIII".to_string(), Solution::int_to_roman(528));

        assert_eq!("MDCLXII".to_string(), Solution::int_to_roman(1662));
    }
}
