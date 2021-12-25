use std::{f32, fs, io};
use std::fs::File;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::io::{BufRead, Write};
use std::path::Path;
use gep2_lib::chromosome::Chromosome;
use gep2_lib::fitness_evaluator;
use gep2_lib::fitness_evaluator::{FitnessEvaluator, FitnessFunction};
use gep2_lib::population::Population;


fn main() {
    let v = read_data();
    let inputs_cnt = 5;
    let mut deltas = Vec::with_capacity(v.len()-1);
    for i in 1..v.len(){ deltas.push(v[i] - v[i-1]) }
    let fe = get_fitness_function(deltas,inputs_cnt,0.75);

    // let mut long_results = Vec::with_capacity(v.len()-1-inputs_cnt);
    // for i in inputs_cnt+1..v.len(){
    //     long_results.push(v[i] - v[i-1])
    // }
    // let mut inputs = Vec::with_capacity(v.len()-1-inputs_cnt);
    // for i in 1..v.len()-1{
    //     inputs.push((v[i] - v[i-1]).signum())
    // }
    // let m = get_matrix(&inputs,inputs_cnt);
    // let fe = FitnessFunction::new(m,long_results,0.75);

    let p = &mut Population::new(30,5, 7, inputs_cnt, 0.4, 0.3);
    let passes = 80;
    let stat = &p.search(&fe,passes);
    for i in 0..stat.len(){
        println!("({}) - max. fitness : {}, avg. fitness : {}", i+1, stat[i].0, stat[i].1);
    }
    println!("{}", stat[stat.len()-1].2);

    let chr = &p.chromosomes[stat[stat.len()-1].3];
    let train_equity = chr.equity(true, &fe);
    let test_equity = chr.equity(false, &fe);

    let mut f = File::create("train_eqt").expect("Unable to create file");
    for i in 0..train_equity.len() {
        writeln!(&mut f, "{},", train_equity[i]).unwrap();
    }

    let mut f = File::create("test_eqt").expect("Unable to create file");
    for i in 0..test_equity.len() {
        writeln!(&mut f, "{},", test_equity[i]).unwrap();
    }
}

pub fn read_data()->Vec<f32>{
    let mut data = Vec::new();
    if let Ok(lines) = read_lines("./data.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let split:Vec<&str> = ip.as_str().split(",").collect();
                data.push(split[0].parse::<f32>().unwrap());
            }
        }
    }
    data
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_fitness_function(deltas:Vec<f32>, inputs_cnt:usize, train_fraction:f32) -> FitnessFunction{
    let len = deltas.len();
    let mut matrix = Vec::with_capacity(len- inputs_cnt -1);
    let mut long_results = Vec::with_capacity(len- inputs_cnt -1);
    for i in inputs_cnt..len{
       let mut row = Vec::with_capacity(inputs_cnt);
        for j in i- inputs_cnt..i{
            row.push(deltas[j].signum())
        }
        long_results.push(deltas[i]);
        matrix.push(row);
    }
    FitnessFunction::new(matrix,long_results,train_fraction)
}

// fn get_matrix(inputs:&Vec<f32>, columns:usize) -> Vec<Vec<f32>>{
//     let len = inputs.len();
//     let start = columns;
//     let mut v = Vec::new();
//     for i in start..len+1{
//        let mut vv = Vec::with_capacity(columns);
//         for j in i-columns..i{
//             vv.push(inputs[j])
//         }
//         v.push(vv)
//     }
//     v
// }


