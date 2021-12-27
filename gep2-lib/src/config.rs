use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub inputs_cnt: usize,
    pub population_size: usize,
    pub nbr_of_genes: usize,
    pub head_length: usize,
    pub transposition_probability: f32,
    pub mutation_probability: f32,
    pub passes: usize,
    pub train_fraction: f32,
    pub data_path: String
}
