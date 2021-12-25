use serde::Deserialize;
use std::{
    fs::{
        read_to_string,
        File
    },
    f32,
    io::{
        self,
        BufRead,
        Write
    },
    path::Path
};
use gep2_lib::{
    fitness_evaluator::FitnessFunction,
    population::Population
};

#[derive(Deserialize)]
struct Config {
    inputs_cnt: usize,
    population_size: usize,
    nbr_of_genes: usize,
    head_length: usize,
    transposition_probability: f32,
    mutation_probability: f32,
    passes: usize,
    train_fraction: f32
}

fn main() {
    let toml_config_str = read_to_string("./driver_config.toml").unwrap();
    let config: Config = toml::from_str(&toml_config_str).unwrap();

    let v = read_data();
    let mut deltas = Vec::with_capacity(v.len()-1);
    for i in 1..v.len(){ deltas.push(v[i] - v[i-1]) }
    let fe = get_fitness_function(deltas, config.inputs_cnt, config.train_fraction);

    let p = &mut Population::new(config.population_size, config.nbr_of_genes, config.head_length,
                                 config.inputs_cnt,
                                 config.transposition_probability, config.mutation_probability);
    let stat = &p.search(&fe,config.passes);
    for i in 0..stat.len(){
        println!("({}) - max. fitness : {}, avg. fitness : {}", i+1, stat[i].0, stat[i].1);
    }
    println!("{}", stat[stat.len()-1].2);

    let chr = &p.chromosomes[stat[stat.len()-1].3];
    let train_equity = chr.equity(true, &fe);
    let test_equity = chr.equity(false, &fe);

    let mut f = File::create("train_eqt.txt").expect("Unable to create file");
    for i in 0..train_equity.len() {
        writeln!(&mut f, "{},", train_equity[i]).unwrap();
    }

    let mut f = File::create("test_eqt.txt").expect("Unable to create file");
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



