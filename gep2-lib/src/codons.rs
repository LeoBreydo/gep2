use crate::functions::Function;
use crate::state_functions::{Collector, Delay, Diff};
use crate::terminal::Terminal;

pub enum Codon {
    Function(Function),
    Delay(Delay),
    Collector(Collector),
    Diff(Diff),
    Terminal(Terminal),
}

impl Clone for Codon {
    fn clone(&self) -> Self {
        match self {
            Codon::Function(ref f) => Codon::Function(Function::new(f.fd)),
            Codon::Delay(ref d) => Codon::Delay(Delay::new()),
            Codon::Collector(ref c) => Codon::Collector(Collector::new()),
            Codon::Diff(ref d) => Codon::Diff(Diff::new()),
            Codon::Terminal(ref t) => Codon::Terminal(Terminal::new(t.i)),
        }
    }
}

impl Codon{
    pub fn get_symbol(&self) -> &str {
        match self{
            Codon::Function(ref f) => f.fd.symbol,
            Codon::Delay(ref d) => d.symbol,
            Codon::Collector(ref c) => c.symbol,
            Codon::Diff(ref d) => d.symbol,
            Codon::Terminal(ref t) => &*t.symbol
        }
    }
    pub fn is_terminal(&self) -> bool {
        match self{
            Codon::Terminal(ref _t) => true,
            _ => false
        }
    }
    pub fn get_arity(&self) -> u8 {
        match self{
            Codon::Function(ref f) => f.fd.arity,
            Codon::Terminal(ref _t) => 0,
            _ => 1
        }

    }
    pub fn evaluate(&mut self, x: f32, y: f32, args: &Vec<f32>) -> f32 {
        match self{
            Codon::Function(ref f) => (f.fd.op)(x,y),
            Codon::Delay(ref mut d) => d.Do(x),
            Codon::Collector(ref mut c) => c.Do(x),
            Codon::Diff(ref mut d) => d.Do(x),
            Codon::Terminal(ref mut t) => t.Do(args)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::functions::REGISTRY;
    use super::*;

    #[test]
    fn terminal_evaluate_test() {
        let mut t = Codon::Terminal(Terminal::new(1));
        assert_eq!(t.evaluate(100.0,100.0,&vec![1.0,-1.0,0.0]), -1.0);
    }

    #[test]
    fn function_evaluate_test() {
        let mut f = Codon::Function(Function::new(&(REGISTRY[0])));
        assert_eq!(f.evaluate(-1.0,1.0,&vec![-1.0,1.0,0.0]), -1.0);
    }

    #[test]
    fn delay_evaluate_test() {
        let mut f = Codon::Delay(Delay::new());
        assert_eq!(f.evaluate(-1.0,1.0,&vec![-1.0,1.0,0.0]), 0.0);
        assert_eq!(f.evaluate(1.0,1.0,&vec![-1.0,1.0,0.0]), -1.0);
    }
}




