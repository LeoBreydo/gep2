use std::{f32, fs::{
    File,
    read_to_string
}, fs, io::{
    self,
    BufRead,
    Write
}, path::Path};
use gep2_lib::{
    fitness_evaluator::FitnessFunction,
    population::Population,
    statistics::Statistics,
    config::Config
};
use chrono::prelude::*;

fn main() {
    // setup
    let toml_config_str = read_to_string("./driver_config.toml").unwrap();
    let conf: Config = toml::from_str(&toml_config_str).unwrap();

    let v = read_data(conf.data_path);
    let mut deltas = Vec::with_capacity(v.len()-1);
    for i in 1..v.len(){ deltas.push(v[i] - v[i-1]) }
    let fe = get_fitness_function(deltas, conf.inputs_cnt, conf.train_fraction);

    let p = &mut Population::new(conf.population_size, conf.nbr_of_genes, conf.head_length,
                                 conf.inputs_cnt, conf.max_delay,
                                 conf.transposition_probability, conf.mutation_probability);
    // search
    let stat = &p.search(&fe, conf.passes);
    for i in 0..stat.len(){
        println!("({}) - max. fitness : {}, avg. fitness : {}", i+1, stat[i].0, stat[i].1);
    }
    println!("{}", stat[stat.len()-1].2);

    // reporting
    save_results(&fe, p, stat);
}

fn save_results(fe: &FitnessFunction, p: &mut Population, stat: &Vec<(f32, f32, String, usize)>) {
    const DATE_FORMAT_STR: &'static str = "%Y%m%d%H%M%S";
    let chr = &p.chromosomes[stat[stat.len() - 1].3];
    let train_equity = chr.equity(p.max_delay, true, fe);
    let train_stat = Statistics::new(&train_equity);
    let test_equity = chr.equity(p.max_delay, false, fe);
    let test_stat = Statistics::new(&test_equity);

    let path = format!("./results_{}", Utc::now().format(DATE_FORMAT_STR).to_string());
    fs::create_dir(&path);

    let mut f = File::create(format!("{}/{}", path, "train_eqt.txt")).expect("Unable to create file");
    for i in 0..train_equity.len() {
        writeln!(&mut f, "{},", train_equity[i]).unwrap();
    }
    let mut f = File::create(format!("{}/{}", path, "test_eqt.txt")).expect("Unable to create file");
    for i in 0..test_equity.len() {
        writeln!(&mut f, "{},", test_equity[i]).unwrap();
    }
    let mut f = File::create(format!("{}/{}", path, "train_stat.txt")).expect("Unable to create file");
    let toml = toml::to_string(&train_stat).unwrap();
    write!(&mut f, "{}", toml);
    let mut f = File::create(format!("{}/{}", path, "test_stat.txt")).expect("Unable to create file");
    let toml = toml::to_string(&test_stat).unwrap();
    write!(&mut f, "{}", toml);

    fs::copy("./driver_config.toml", format!("{}/{}", path, "driver_config.toml"));
}

pub fn read_data(path: String)->Vec<f32>{
    let mut data = Vec::new();
    if let Ok(lines) = read_lines(path) {
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




