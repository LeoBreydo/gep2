use serde::Serialize;

#[derive(Serialize)]
pub struct Statistics {
    result: f32,
    win: f32,
    loss: f32,
    profit_factor: f32,
    count: u32,
    percent_profitable: f32,
    avg_win: f32,
    avg_loss: f32
}

impl Statistics{
    pub fn new(equity: &Vec<f32>) -> Self{
        let mut prev = 0.0;
        let mut win = 0.0;
        let mut loss = 0.0;
        let mut win_cnt = 0;
        let cnt = equity.len() as u32;
        for i in 0..equity.len(){
            let temp = equity[i];
            let res = temp - prev;
            prev = temp;
            if res > 0.0 {
                win_cnt += 1;
                win += res;
            }
            else{
                loss -= res;
            }
        }
        Statistics{
            result: win-loss,
            win,
            loss,
            profit_factor: if loss == 0.0{1000000.0} else {win/loss},
            count: cnt,
            percent_profitable: 100.0 * (win_cnt as f32)/(cnt as f32),
            avg_win: if win_cnt == 0 {0.0} else {win/(win_cnt as f32)},
            avg_loss: if win_cnt == cnt {0.0} else {loss/((cnt - win_cnt) as f32)}
        }
    }
}
