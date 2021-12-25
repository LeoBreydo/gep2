use std::cell::Cell;

pub struct FunctionDescription {
    pub symbol : &'static str,
    pub arity: u8,
    pub op : fn(f32, f32) -> f32
}

pub fn av(x: f32, y: f32) -> f32{
    (x+y)/2.0
}
pub fn ne(x: f32, _y: f32) -> f32{
    -x
}

pub const FREGISTRY: &'static [FunctionDescription] = &[
    FunctionDescription {
        arity: 2,
        op: f32::min,
        symbol: "Min"
    },
    FunctionDescription {
        arity: 2,
        op: f32::max,
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

pub struct Function{
    pub fd: &'static FunctionDescription,
    // to hide mutability
    pub first_arg_position:Cell<usize>
}
impl Function{
    pub fn new(fd: &'static FunctionDescription) -> Self{
        Function{ fd, first_arg_position:Cell::new(0) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_length_test() {
        assert_eq!(FREGISTRY.len(), FN_NUM);
    }
    #[test]
    fn registry_mi_test() {
        assert_eq!((FREGISTRY[0].op)(1.0, -1.0), -1.0);
    }
    #[test]
    fn registry_ne_test() {
        assert_eq!((FREGISTRY[FN_NUM-1].op)(1.0, 1.0), -1.0);
    }
    #[test]
    fn registry_av_test() {
        assert_eq!((FREGISTRY[FN_NUM-2].op)(1.0, -1.0), 0.0);
    }

}



