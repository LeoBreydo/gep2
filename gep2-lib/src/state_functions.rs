use std::cell::Cell;

pub struct StateFunctionDescription{
    pub symbol : &'static str,
    pub op : fn(f32, f32) -> f32,
}
pub fn delay(_curr: f32, prev: f32) -> f32{ prev }
pub fn ma2(curr: f32, prev: f32) -> f32{ 0.5*(curr + prev) }
pub fn diff(curr: f32, prev: f32) -> f32{
    let temp = curr - prev;
    if temp > 1.0 {1.0} else if temp < -1.0 {-1.0} else {temp}
}

pub const SFREGISTRY: &'static [StateFunctionDescription] = &[
    StateFunctionDescription {
        op: delay,
        symbol: "Delay"
    },
    StateFunctionDescription {
        op: ma2,
        symbol: "Ma2"
    },
    StateFunctionDescription {
        op: diff,
        symbol: "Diff"
    },
];

pub const SFN_NUM: usize = 3;

pub struct StateFunction{
    pub fd: &'static StateFunctionDescription,
    pub buf: Cell<f32>,
    pub first_arg_position:Cell<usize>
}
impl StateFunction{
    pub fn new(fd: &'static StateFunctionDescription) -> Self{
        StateFunction{ fd, buf: Cell::new(0.0), first_arg_position:Cell::new(0) }
    }
    pub fn eval(&self, curr:f32)-> f32{
        let ret = (self.fd.op)(curr,self.buf.get());
        self.buf.set(curr);
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delay_test() {
        let mut d = StateFunction::new(&SFREGISTRY[0]);
        let x1 = d.eval(1.0);
        let x2 = d.eval(0.0);
        assert_eq!(0.0, x1);
        assert_eq!(1.0, x2);
    }

    #[test]
    fn ma2_test() {
        let mut d = StateFunction::new(&SFREGISTRY[1]);
        let x1 = d.eval(1.0); // 0.5
        let x2 = d.eval(1.0); // 1
        let x3 = d.eval(-1.0); // 0
        assert_eq!(0.5, x1);
        assert_eq!(1.0, x2);
        assert_eq!(0.0, x3);
    }

    #[test]
    fn diff_test() {
        let mut d = StateFunction::new(&SFREGISTRY[2]);
        let x1 = d.eval(1.0); // 1
        let x2 = d.eval(1.0); // 0
        let x3 = d.eval(-1.0); // -1
        assert_eq!(1.0, x1);
        assert_eq!(0.0, x2);
        assert_eq!(-1.0, x3);
    }
}
