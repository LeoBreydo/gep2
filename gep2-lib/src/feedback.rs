use crate::delay_line::DelayLine;

pub struct Feedback {
    pub i: usize,
    pub symbol: String,
}
impl Feedback{
    pub fn new(i:usize) -> Self{
        Feedback{
            i,
            symbol: format!("delay[{}]", i)
        }
    }
    pub fn eval(&self, delay_line: &DelayLine) -> f32 {
        let ret = delay_line.get_shifted_back(self.i);
        if ret < -1.0 {-1.0} else if ret > 1.0 {1.0} else {ret}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feedback_test() {
        let mut dl = DelayLine::new(3);
        let mut f = Feedback::new(1);
        dl.push(0.0);
        dl.push(1.0);
        dl.push(2.0);
        assert_eq!(f.eval(&dl),1.0);
    }
}