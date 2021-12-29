use crate::delay_line::DelayLine;
use crate::feedback::Feedback;
use crate::functions::Function;
use crate::state_functions::StateFunction;
use crate::terminal::Terminal;

pub enum Codon {
    Function(Function),
    StateFunction(StateFunction),
    Terminal(Terminal),
    Feedback(Feedback),
}

impl Clone for Codon {
    fn clone(&self) -> Self {
        match self {
            Codon::Function(ref f) => Codon::Function(Function::new(f.fd)),
            Codon::StateFunction(ref f) => Codon::StateFunction(StateFunction::new(f.fd)),
            Codon::Terminal(ref t) => Codon::Terminal(Terminal::new(t.i)),
            Codon::Feedback((ref f)) => Codon::Feedback(Feedback::new(f.i))
        }
    }
}

impl Codon{
    pub fn get_symbol(&self) -> &str {
        match self{
            Codon::Function(ref f) => f.fd.symbol,
            Codon::StateFunction(ref f) => f.fd.symbol,
            Codon::Terminal(ref t) => &*t.symbol,
            Codon::Feedback(ref f) => &*f.symbol
        }
    }
    pub fn is_terminal(&self) -> bool {
        match self{
            Codon::Terminal(ref _t) => true,
            Codon::Feedback(ref _t) => true,
            _ => false
        }
    }
    pub fn get_arity(&self) -> u8 {
        match self{
            Codon::Function(ref f) => f.fd.arity,
            Codon::Terminal(ref _t) => 0,
            Codon::Feedback(ref _t) => 0,
            _ => 1
        }

    }
    pub fn set_first_arg_position(&self, pos:usize) {
        match self{
            Codon::Function(ref f) => f.first_arg_position.set(pos),
            Codon::StateFunction(ref f) => f.first_arg_position.set(pos),
            _ => { },
        }
    }
    pub fn get_first_arg_position(&self) -> usize{
        match self{
            Codon::Function(ref f) => f.first_arg_position.get(),
            Codon::StateFunction(ref f) => f.first_arg_position.get(),
            _ => 0,
        }
    }
    pub fn evaluate(&self, x: f32, y: f32, args: &Vec<f32>, delay_line: &DelayLine) -> f32 {
        match self{
            Codon::Function(ref f) => (f.fd.op)(x,y),
            Codon::StateFunction(ref f) => f.eval(x),
            Codon::Terminal(ref t) => t.eval(args),
            Codon::Feedback(ref f) => f.eval(delay_line)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::functions::FREGISTRY;
    use crate::state_functions::SFREGISTRY;
    use super::*;

    #[test]
    fn terminal_evaluate_test() {
        let mut t = Codon::Terminal(Terminal::new(1));
        let mut dl = DelayLine::new(3);
        assert_eq!(t.evaluate(100.0,100.0,&vec![1.0,-1.0,0.0],&dl), -1.0);
    }

    #[test]
    fn function_evaluate_test() {
        let mut f = Codon::Function(Function::new(&(FREGISTRY[0])));
        let mut dl = DelayLine::new(3);
        assert_eq!(f.evaluate(-1.0,1.0,&vec![-1.0,1.0,0.0], &dl), -1.0);
    }

    #[test]
    fn delay_evaluate_test() {
        let mut f = Codon::StateFunction(StateFunction::new(&SFREGISTRY[0]));
        let mut dl = DelayLine::new(3);
        assert_eq!(f.evaluate(-1.0,1.0,&vec![-1.0,1.0,0.0], &dl), 0.0);
        assert_eq!(f.evaluate(1.0,1.0,&vec![-1.0,1.0,0.0]), -1.0);
    }

    #[test]
    fn feedback_test() {
        let mut t = Codon::Feedback(Feedback::new(1));
        let mut dl = DelayLine::new(3);
        dl.push(0.0);
        dl.push(1.0);
        dl.push(2.0);
        assert_eq!(f.evaluate(-1.0,1.0,&vec![-1.0,1.0,0.0], &dl), 0.0);
    }
}




