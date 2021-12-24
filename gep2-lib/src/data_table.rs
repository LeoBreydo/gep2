pub struct DataTable{
    pub args:Vec<Vec<f32>>,
    pub results: Vec<f32>,
    pub rows:usize,
    pub cols:usize,
}

impl DataTable{
    pub fn new(a:Vec<Vec<f32>>,r:Vec<f32>) -> Option<Self>{
        let (al,rl) = (a.len(),r.len());
        if al == 0 {return None}
        let arl = a[0].len();
        if arl == 0 {return None}
        if al != rl {return None}
        for row in a.iter(){
            if row.len()  != arl {return None}
        }
        Some(DataTable{args:a,results:r,rows:al,cols:arl})
    }
    pub fn get_data_row(&self, idx:usize) -> Option<(&Vec<f32>, f32)> {
        if idx < self.rows {Some((&self.args[idx],self.results[idx]))} else{None}
    }
}