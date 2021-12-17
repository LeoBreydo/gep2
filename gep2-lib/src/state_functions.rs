pub struct Delay {
    pub buf: f32,
    pub first_arg_position:usize,
    pub symbol:  &'static str,
}
impl Delay{
    pub fn new() -> Self{
        Delay{
            buf:0.0,
            first_arg_position:0,
            symbol: "Delay"
        }
    }
    pub fn eval(&mut self, x:f32) -> f32{
        let ret = self.buf;
        self.buf = x;
        ret
    }
}
pub struct Collector {
    pub buf: f32,
    pub first_arg_position:usize,
    pub symbol:  &'static str,
}
impl Collector {
    pub fn new() -> Self{
        Collector {
            buf:0.0,
            first_arg_position:0,
            symbol: "Collect"
        }
    }
    pub fn eval(&mut self, x:f32) -> f32{
        let temp = (self.buf + x)/2.0;
        self.buf = x;
        if temp > 1.0 {1.0} else if temp < -1.0 {-1.0} else {temp}
    }
}
pub struct Diff {
    pub buf:f32,
    pub first_arg_position:usize,
    pub symbol:  &'static str,
}
impl Diff{
    pub fn new() -> Self{
        Diff{
            buf:0.0,
            first_arg_position:0,
            symbol: "Diff"
        }
    }
    pub fn eval(&mut self, x:f32) -> f32{
        let temp = x - self.buf;
        self.buf = x;
        if temp > 1.0 {1.0} else if temp < -1.0 {-1.0} else {temp}
    }
}

pub const SFN_NUM: usize = 3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delay_test() {
        let mut d = Delay::new();
        let x1 = d.eval(1.0);
        let x2 = d.eval(0.0);
        assert_eq!(0.0, x1);
        assert_eq!(1.0, x2);
    }

    #[test]
    fn collector_test() {
        let mut d = Collector::new();
        let x1 = d.eval(1.0); // 0.5
        let x2 = d.eval(1.0); // 1
        let x3 = d.eval(-1.0); // 0
        assert_eq!(0.5, x1);
        assert_eq!(1.0, x2);
        assert_eq!(0.0, x3);
    }

    #[test]
    fn diff_test() {
        let mut d = Diff::new();
        let x1 = d.eval(1.0); // 1
        let x2 = d.eval(1.0); // 0
        let x3 = d.eval(-1.0); // -1
        assert_eq!(1.0, x1);
        assert_eq!(0.0, x2);
        assert_eq!(-1.0, x3);
    }
}
