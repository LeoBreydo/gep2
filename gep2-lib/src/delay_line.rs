pub struct DelayLine{
    pub capacity:usize,
    idx:usize,
    buff: Vec<f32>
}

impl DelayLine{
    pub fn new(capacity: usize) -> Self{
        let c = if capacity < 1 {1} else {capacity};
        let mut buff = Vec::with_capacity(c);
        for _i in 0..c{
            buff.push(0.0)
        }
        DelayLine{capacity:c,idx:0,buff}
    }
    pub fn push(&mut self,v:f32){
        self.buff[self.idx] = v;
        self.idx += 1;
        if self.idx == self.capacity {self.idx = 0}
    }
    pub fn delayed(&self, shift:usize) -> f32{
        let i = (shift+1)%self.capacity;
        self.buff[if i > self.idx {self.idx+self.capacity - i} else {self.idx-i}]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_test() {
        let mut dl = DelayLine::new(4);

        dl.push(1.0);
        dl.push(2.0);
        dl.push(3.0);
        dl.push(4.0);
        dl.push(5.0);
        assert_eq!(dl.delayed(0),5.0);
        assert_eq!(dl.delayed(1),4.0);
        assert_eq!(dl.delayed(2),3.0);
        assert_eq!(dl.delayed(3),2.0);
    }
}