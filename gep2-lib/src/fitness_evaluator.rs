use std::f32;
use crate::data_table::DataTable;
use crate::delay_line::DelayLine;

pub trait FitnessEvaluator {
    fn evaluate<'a>(&self, max_delay:usize, func: Box<dyn Fn(&Vec<f32>, &DelayLine) -> f32 +'a>) -> f32;
    fn equity<'a>(&self, max_delay: usize, train:bool, func: Box<dyn Fn(&Vec<f32>, &DelayLine) -> f32 +'a>) -> Vec<f32>;
}

pub struct FitnessFunction{
    data_table:DataTable,
    end_of_train_data:usize,
    train_perfect_equity:f32,
    test_perfect_equity: f32
}

impl FitnessFunction{
    pub fn new(a:Vec<Vec<f32>>,r:Vec<f32>, train_fraction:f32) -> Self{
        let len = r.len();
        let mut train_e = 0.0f32;
        let end_of_train_data = ((len as f32)*train_fraction) as usize;
        for i in 0..end_of_train_data{
            train_e += r[i].abs()
        }
        let mut test_e = 0.0f32;
        for i in end_of_train_data..len{
            test_e += r[i].abs()
        }
        let data_table = DataTable::new(a,r).unwrap();
        FitnessFunction{data_table,end_of_train_data,
            train_perfect_equity: train_e, test_perfect_equity: test_e}
    }
}
impl FitnessEvaluator for FitnessFunction {
    fn evaluate<'a>(&self, max_delay:usize, func: Box<dyn Fn(&Vec<f32>, &DelayLine) -> f32 +'a>) -> f32 {
        let mut res = 0.0f32;
        let mut dl = DelayLine::new(max_delay);
        for i in 0..self.end_of_train_data {
            let row = self.data_table.get_data_row(i).unwrap();
            let s =  func(row.0,&dl);
            res += row.1 * s;
            dl.push(s)
        };
        if res > 0.0 {res/self.train_perfect_equity} else {0.0}
    }

    fn equity<'a>(&self, max_delay: usize, train: bool, func: Box<dyn Fn(&Vec<f32>, &DelayLine) -> f32 + 'a>) -> Vec<f32> {
        let mut ret = Vec::new();
        let mut res = 0.0f32;
        let mut dl = DelayLine::new(max_delay);
        if train{
            for i in 0..self.end_of_train_data {
                let row = self.data_table.get_data_row(i).unwrap();
                let s = func(row.0, &dl);
                res += s*row.1;
                dl.push(s);
                ret.push(res);
            }
        }
        else{
            for i in self.end_of_train_data..self.data_table.rows {
                let row = self.data_table.get_data_row(i).unwrap();
                let s = func(row.0, &dl);
                res += s*row.1;
                dl.push(s);
                ret.push(res);
            }
        }
        ret
    }
}

