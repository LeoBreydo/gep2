
pub struct FunctionDescription {
    pub symbol : &'static str,
    pub arity: u8,
    pub op : fn(f32, f32) -> f32
}

pub fn mi(x: f32, y: f32) -> f32{
    f32::min(x, y)
}
pub fn ma(x: f32, y: f32) -> f32{
    f32::max(x, y)
}
pub fn av(x: f32, y: f32) -> f32{
    (x+y)/2.0
}
pub fn ne(x: f32, _y: f32) -> f32{
    -1.0*x
}

pub const REGISTRY: &'static [FunctionDescription] = &[
    FunctionDescription {
        arity: 2,
        op: mi,
        symbol: "Min"
    },
    FunctionDescription {
        arity: 2,
        op: ma,
        symbol: "Max"
    },
    FunctionDescription {
        arity: 2,
        op: av,
        symbol: "Avg"
    },
    FunctionDescription {
        arity: 1,
        op: ne,
        symbol: "Neg"
    },
];

pub const FN_NUM: usize = 4;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_length_test() {
        assert_eq!(REGISTRY.len(), FN_NUM);
    }
    #[test]
    fn registry_mi_test() {
        assert_eq!((REGISTRY[0].op)(1.0,-1.0), -1.0);
    }
    #[test]
    fn registry_ne_test() {
        assert_eq!((REGISTRY[FN_NUM-1].op)(1.0,1.0), -1.0);
    }
    #[test]
    fn registry_av_test() {
        assert_eq!((REGISTRY[FN_NUM-2].op)(1.0,-1.0), 0.0);
    }

}



