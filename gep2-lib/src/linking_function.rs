pub struct LinkingFunction {
    pub symbol : &'static str,
}

impl LinkingFunction {
    pub fn evaluate(&self, args: Vec<f32>) -> f32{
        args.iter().sum::<f32>().signum()
    }
}

pub const LF: LinkingFunction = LinkingFunction { symbol: "Maj" };