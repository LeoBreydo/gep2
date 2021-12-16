use rand::prelude::ThreadRng;
use rand::Rng;
use gep2_lib::chromosome::{Chromosome};



fn main() {
    let mut rg = rand::thread_rng();
    let chromosome = Chromosome::new(&mut rg,3,3,5);
    let k_representation = chromosome.k_string();
    dbg!(k_representation);
}
