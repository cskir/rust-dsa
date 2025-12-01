// leetcode Medium 155. Min Stack
struct MinStack {
    stack: Vec<i32>,
    min_so_far: Vec<i32>,
}

// Methods pop, top and getMin operations will always be called on non-empty stacks.
impl MinStack {
    #[allow(dead_code)]
    fn new() -> Self {
        MinStack {
            stack: vec![],
            min_so_far: vec![],
        }
    }

    #[allow(dead_code)]
    fn push(&mut self, val: i32) {
        self.stack.push(val);

        if self.min_so_far.is_empty() {
            self.min_so_far.push(val);
        } else {
            let m = *self.min_so_far.last().unwrap();
            if val < m {
                self.min_so_far.push(val);
            } else {
                self.min_so_far.push(m);
            }
        }
    }

    #[allow(dead_code)]
    fn pop(&mut self) {
        self.min_so_far.pop().unwrap();
        self.stack.pop().unwrap();
    }

    #[allow(dead_code)]
    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    #[allow(dead_code)]
    fn get_min(&self) -> i32 {
        *self.min_so_far.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let mut k = MinStack::new();
        k.push(-2);
        k.push(0);
        k.push(-3);

        assert_eq!(k.get_min(), -3);
        k.pop();
        assert_eq!(k.top(), 0);
        assert_eq!(k.get_min(), -2);
    }

    #[test]
    fn test_2() {
        let mut k = MinStack::new();
        k.push(-2);
        k.push(0);
        k.push(-1);

        assert_eq!(k.get_min(), -2);
        assert_eq!(k.top(), -1);
        k.pop();
        assert_eq!(k.get_min(), -2);
    }
}
