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
    pub fn Do(&mut self, x:f32) -> f32{
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
    pub fn Do(&mut self, x:f32) -> f32{
        let mut temp = self.buf + x;
        self.buf = if temp > 1.0 {1.0} else if temp < -1.0 {-1.0} else {temp};
        self.buf
    }
}
pub struct Diff {
    pub buf: f32,
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
    pub fn Do(&mut self, x:f32) -> f32{
        let mut temp = x - self.buf;
        self.buf = if temp > 1.0 {1.0} else if temp < -1.0 {-1.0} else {temp};
        self.buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delay_test() {
        let mut d = Delay::new();
        let x1 = d.Do(1.0);
        let x2 = d.Do(0.0);
        assert_eq!(0.0, x1);
        assert_eq!(1.0, x2);
    }

    #[test]
    fn collector_test() {
        let mut d = Collector::new();
        let x1 = d.Do(1.0);
        let x2 = d.Do(1.0);
        let x3 = d.Do(-1.0);
        assert_eq!(1.0, x1);
        assert_eq!(1.0, x2);
        assert_eq!(0.0, x3);
    }

    #[test]
    fn diff_test() {
        let mut d = Diff::new();
        let x1 = d.Do(1.0);
        let x2 = d.Do(1.0);
        let x3 = d.Do(-1.0);
        assert_eq!(1.0, x1);
        assert_eq!(0.0, x2);
        assert_eq!(-1.0, x3);
    }
}
