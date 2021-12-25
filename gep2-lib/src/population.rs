use rand::prelude::ThreadRng;
use rand::Rng;
use crate::chromosome::Chromosome;
use crate::fitness_evaluator::FitnessEvaluator;

pub struct Population {
    pub rng: ThreadRng,
    pub chromosomes: Vec<Chromosome>,
    pub size:usize,
    pub gene_nbr:usize,
    pub hl:usize,
    pub num_args:usize,
    denominator: f32,
    transposition_probability: f32, // def. val. 0.3
    mutation_probability: f32, // def. val. 0.2
}

impl Population{
    pub fn new(mut size:usize,gn:usize, mut hl:usize, mut num_args:usize,
               mut transposition_probability : f32,
               mut mutation_probability : f32) -> Self{
        if num_args == 0 {num_args = 1}
        let mut d = 0.0;
        let mut rg = rand::thread_rng();
        if size < 2 {size = 2}
        if hl < 1 {hl = 1}
        let mut chr: Vec<Chromosome> = Vec::with_capacity(size);
        for _i in 0..size{
            d += 1.0;
            chr.push(Chromosome::new(&mut rg, gn, hl, num_args))
        }
        if transposition_probability > 1.0 {
            transposition_probability = 1.0;
        }
        if mutation_probability > 1.0{
            mutation_probability = 1.0;
        }
        if mutation_probability <= 0.0 {
            mutation_probability = 0.0;
        }
        Population{
            rng: rg,
            chromosomes: chr,
            size,
            gene_nbr:gn,
            hl,
            num_args,
            denominator:d,
            transposition_probability,
            mutation_probability
        }
    }

    pub fn search<'a>(&'a mut self, fe: &'a impl FitnessEvaluator, g:usize) -> Vec<(f32, f32, String, usize)>{
        let mut stat: Vec<(f32,f32, String, usize)> = Vec::with_capacity(g);
        let mut r : Option<(f32,f32, String, usize)>;
        // initialization
        loop {
            r = self.evaluate(fe);
            if r != None {break}
            // total fitness is about zero - reinitialize
            for i in 0..self.size{
                self.chromosomes[i] = Chromosome::new(&mut self.rng, self.gene_nbr,  self.hl, self.num_args);
            }
        }
        stat.push(r.unwrap());

        // gp-search
        for i in 0..g {
            self.update();
            stat.push(self.evaluate(fe).unwrap());
            if stat[i+1].0 > 0.95 {break}
        }
        stat
    }

    // because of elitism we can return None within phase of initialization only
    pub fn evaluate<'a>(&'a self, fe: &'a impl FitnessEvaluator) -> Option<(f32, f32, String, usize)> {
        let mut total = 0.0;
        let mut mf=-1.0;
        let mut ii = 0;
        let mut i = 0;
        for chr in  & self.chromosomes{
            let f = chr.pass(fe);
            if f > mf{
                mf = f;
                ii = i;
            }
            i += 1;
            total += f;
        }
        if total < 0.001 {return None;}
        let avg = total/self.denominator;
        for i in 0..self.size{
            let c = &self.chromosomes[i];
            let fi = c.fitness.get();
            c.fitness.set(fi/total);
        }
        Some((mf,avg,self.chromosomes[ii].k_string(), ii))
    }

    fn update(&mut self) {
        let (wheel, wheel_ids, id_of_best) = self.build_roulette_wheel();
        let mut next_generation: Vec<Chromosome> = Vec::with_capacity(self.size);
        // copy chromosomes with positive fitness (optionally mutated) to new generation
        for _i in 0..self.size{
            let test = self.rng.gen_range(0.0..1.0);
            for ii in 0..wheel.len() {
                if test < wheel[ii] {
                    if self.mutation_probability > 0.0 {
                        next_generation.push(self.chromosomes[wheel_ids[ii]].mutation(&mut self.rng, self.num_args, self.mutation_probability));
                    }
                    else{
                        next_generation.push(self.chromosomes[wheel_ids[ii]].copy_to_new_generation());
                    }
                    break;
                }
            }
        }
        // apply root_transposition to chromosomes in a new generation as per a given probability
        for i in 0..self.size{
            if self.transposition_probability <= 0.0 {
                continue;
            }
            next_generation[i].root_transposition(&mut self.rng, self.transposition_probability);
        }
        // apply two points crossover to chromosomes in a new generation
        for i in (0..self.size).step_by(2){
            Chromosome::two_points_crossover(&mut next_generation, i, i+1, &mut self.rng);
        };
        // some elitism :)
        next_generation[0] = self.chromosomes[id_of_best].copy_to_new_generation();
        // update current population
        self.chromosomes = next_generation;
    }

    fn build_roulette_wheel(&mut self) -> (Vec<f32>, Vec<usize>, usize) {
        let mut wheel: Vec<f32> = Vec::with_capacity(self.size);
        let mut wheel_idx: Vec<usize> = Vec::with_capacity(self.size);
        let mut prev = 0.0;
        let mut max_ind = 0;
        let mut max_f = -1.0;
        for i in 0..self.size {
            let f = self.chromosomes[i].fitness.get();
            if f < 0.0001 { continue }
            if f > max_f {
                max_f = f;
                max_ind = i;
            }
            prev += f;
            wheel.push(prev);
            wheel_idx.push(i);
        }
        (wheel, wheel_idx, max_ind)
    }

    pub fn show_pass_results(&self){
        for i in 0..self.size {
            println!("chr #{} : {}",i,self.chromosomes[i].fitness.get())
        }
    }
}