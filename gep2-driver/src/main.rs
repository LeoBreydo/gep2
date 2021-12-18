use gep2_lib::chromosome::{Chromosome};

fn main() {
    let mut rg = rand::thread_rng();
    // let mut chr: Vec<Chromosome> = Vec::with_capacity(2);
    // chr.push(Chromosome::new(&mut rg,3,3,5));
    // chr.push(Chromosome::new(&mut rg,3,3,5));
    // dbg!(&chr[0].k_string());
    // dbg!(&chr[1].k_string());
    // Chromosome::two_points_crossover(&mut chr, 0, 1, &mut rg);
    // dbg!(&chr[0].k_string());
    // dbg!(&chr[1].k_string());

    let chromosome = Chromosome::new(&mut rg,3,3,5);
    dbg!(chromosome.k_string());

    let mut chromosome = chromosome.mutation(&mut rg,5,0.14);
    dbg!(chromosome.k_string());

    chromosome.root_transposition(&mut rg, 1.0);
    dbg!(chromosome.k_string());
}
