pub struct Terminal {
    pub i: usize,
    pub symbol: String,
}
impl Terminal{
    pub fn new(i:usize) -> Self{
        Terminal{
            i,
            symbol: format!("args[{}]", i)
        }
    }
    pub fn eval(&self, args: &Vec<f32>) -> f32 {
        let ret = args[self.i];
        if ret < -1.0 {-1.0} else if ret > 1.0 {1.0} else {ret}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn big_neg_test() {
        let args = &vec![-100.0];
        let mut t = Terminal::new(0);
        let v = t.eval(args);
        assert_eq!(-1.0, v);
    }
    #[test]
    fn big_pos_test() {
        let args = &vec![-100.0, 100.0];
        let mut t = Terminal::new(1);
        let v = t.eval(args);
        assert_eq!(1.0, v);
    }
    #[test]
    fn in_range_test() {
        let args = &vec![-100.0, 100.0, 0.5];
        let mut t = Terminal::new(2);
        let v = t.eval(args);
        assert_eq!(0.5, v);
    }
}



