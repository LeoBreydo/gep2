use std::cell::Cell;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::codons::Codon;
use crate::fitness_evaluator::FitnessEvaluator;
use crate::functions::{FN_NUM, Function, REGISTRY};
use crate::linking_function::LF;
use crate::state_functions::{Collector, Delay, Diff, SFN_NUM};
use crate::terminal::Terminal;

pub struct Chromosome {
    head_size:usize,
    nbr_of_genes:usize,

    pub codons: Vec<Codon>,
    // to hide mutability
    pub fitness: Cell<f32>,
}
impl Chromosome{
    // initialization
    pub fn new(mut rng: &mut ThreadRng, mut gene_nbr:usize, mut head_length: usize, mut args_nbr:usize) -> Self{
        if gene_nbr < 1 { gene_nbr = 1}
        if head_length < 1 { head_length = 1}
        if args_nbr < 1 { args_nbr = 1}

        let gl = 2* head_length + 1;
        let mut codons: Vec<Codon> = Vec::with_capacity(gene_nbr *gl);
        Self::initialize_codons(&mut rng, &mut codons, gene_nbr, head_length, args_nbr);
        Chromosome { codons, head_size: head_length, nbr_of_genes: gene_nbr, fitness:Cell::new(0.0) }
    }
    fn initialize_codons(mut rng: &mut ThreadRng, arr: &mut Vec<Codon>, gn:usize, hl:usize, na:usize){
        let gl = 2*hl+1;
        for i in 0..gn{
            let start = i*gl;
            // first codon in gene must be non-terminal
            arr.push(Self::create_non_terminal(rng));
            for _j in start+1..start+hl{
                Self::push_head_codon(&mut rng, arr, na);
            }
            for _j in start+hl..start+gl{
                arr.push(Codon::Terminal(Terminal::new(rng.gen::<usize>() % na)));
            }
        }
    }
    fn push_head_codon(rng: &mut ThreadRng, arr: &mut Vec<Codon>, na:usize) {
        let r = rng.gen::<usize>() % 10;
        if r < 2 {
            // terminal creation probability is 20%
            arr.push(Codon::Terminal(Terminal::new(rng.gen::<usize>() % na)));
        } else {
            let c = Self::create_non_terminal(rng);
            arr.push(c);
        }
    }
    fn create_non_terminal(rng: &mut ThreadRng) -> Codon {
        let i = rng.gen::<usize>() % (FN_NUM + SFN_NUM);
        let c = if i < 4 {
            Codon::Function(Function::new(&(REGISTRY[i])))
        } else {
            match i {
                4 => Codon::Delay(Delay::new()),
                5 => Codon::Collector(Collector::new()),
                _ => Codon::Diff(Diff::new())
            }
        };
        c
    }

    pub fn copy_to_new_generation(&self) ->Chromosome{
        let len = self.codons.len();
        let mut codons: Vec<Codon> = Vec::with_capacity(len);
        for i in 0.. self.codons.len(){
            codons.push(self.codons[i].clone());
        }
        Chromosome { head_size: self.head_size, nbr_of_genes: self.nbr_of_genes, codons, fitness:Cell::new(0.0) }
    }


    // helpers
    pub fn k_string(&self) ->String{
        let len = self.head_size*2+1;
        let mut ret = String::new();
        ret.push_str (LF.symbol);
        for i in 0..self.nbr_of_genes {
            let start = i * len;
            for j in start..start + len {
                ret.push_str(" |");
                let c = &self.codons[j];
                match c {
                    Codon::Terminal(ref t) => ret.push_str(&t.i.to_string()),
                    _ => ret.push_str(&c.get_symbol())
                }
                ret.push('|');
            }
            if i < self.nbr_of_genes-1 {ret.push_str(" //");}
        }
        ret
    }

    // genetic operations
    pub fn mutation(&self, rng: &mut ThreadRng, args_nbr:usize, codon_mutation_probability: f32) -> Chromosome{
        let glen = self.head_size*2+1;
        // codons for mutated chromosome
        let mut codons: Vec<Codon> = Vec::with_capacity(self.codons.len());
        for i in 0..self.nbr_of_genes {
            let start = i * glen;
            //mutate first head position (must be non-terminal)
            if rng.gen_range(0.0..1.0) < codon_mutation_probability{
                codons.push(Self::create_non_terminal(rng));
            }
            else{
                codons.push(self.codons[start].clone());
            }
            // mutate rest of head
            for j in start+1..start + self.head_size {
                if rng.gen_range(0.0..1.0) < codon_mutation_probability{
                    let r = rng.gen::<usize>() % 10;
                    if r < 2 {
                        codons.push(Codon::Terminal(Terminal::new(rng.gen::<usize>() % args_nbr)));
                    } else {
                        codons.push(Self::create_non_terminal(rng));
                    }
                }
                else{
                    codons.push(self.codons[j].clone());
                }
            }
            //mutate tail
            for j in start + self.head_size..start + glen {
                if rng.gen_range(0.0..1.0) < codon_mutation_probability{
                    codons.push(Codon::Terminal(Terminal::new(rng.gen::<usize>() % args_nbr)));
                }
                else{
                    codons.push(self.codons[j].clone());
                }
            }
        }
        Chromosome { codons, head_size:self.head_size, nbr_of_genes:self.nbr_of_genes, fitness: Cell::new(0.0) }
    }
    pub fn root_transposition(&mut self, rng: &mut ThreadRng, transposition_probability : f32) {
        let test = rng.gen_range(0.0..1.0);
        if test >= transposition_probability {return;}
        let glen = self.head_size*2+1;
        // 1) select target gene
        let target_gene = rng.gen::<usize>() % self.nbr_of_genes;
        let mut ip = glen * target_gene;
        // 2) select source gene
        let source_gene = rng.gen::<usize>() % self.nbr_of_genes;
        let ss = glen * source_gene; // source gene head start
        // 3) select starting point of transposon (within head of source gene)
        let mut starting_point = rng.gen::<usize>() % self.head_size + ss;
        //  3.1) if starting point is a root of target gene - move starting point forward, to next codon
        if starting_point == ip {starting_point += 1}
        let se = ss + self.head_size; // source gene tail start
        // try to move starting_point to position of first occurrence of a non-terminal in a given transposon
        // because after transposition modified gene cannot start with a terminal
        loop{
            match self.codons[starting_point]{
                Codon::Terminal(ref _t) => starting_point += 1,
                _ => break,
            }
            // only terminals found -> no transposition
            if starting_point == se {
                // println!("only terminals found -> no transposition");
                return;
            }
        }
        // 4) select transposon end point (between starting point and end of head of source gene - transposon must not cross head/tail border)
        let end_point = starting_point + rng.gen::<usize>() % (se - starting_point);
        // dbg!(starting_point);
        // dbg!(end_point);
        // dbg!(ip);
        // 5) clone transposon starting with a root of target gene
        // because of restriction applied to length of transposon, we can clone codons directly, without using temporary buffer
        for i in starting_point..end_point+1{
            self.codons[ip] = self.codons[i].clone();
            ip += 1;
        }
    }
    pub fn two_points_crossover(chrs: &mut Vec<Chromosome>, i1: usize, i2: usize, rng: &mut ThreadRng) {
        let len = chrs[0].codons.len();
        let point1 = rng.gen::<usize>() % len;
        let mut point2:usize;
        loop{
            point2 = rng.gen::<usize>() % len;
            if point1 != point2 {break}
        }
        let (start,end) = if point1 < point2 {(point1,point2)} else {(point2,point1)};
        // dbg!(start);
        // dbg!(end);
        let (a, b) = chrs.split_at_mut(i2);
        let p1 = &mut a[i1];
        let p2 = &mut b[0];
        for i in start..end+1{
            let (x, y) = (p2.codons[i].clone(), p1.codons[i].clone());
            p1.codons[i] = x;
            p2.codons[i] = y;
        }
    }

    // translation/execution
    pub fn translate<'a>(&'a self) -> Box<dyn Fn(&Vec<f32>) -> f32 + 'a>{
        let glen = 2*self.head_size+1;
        let cnt = self.nbr_of_genes;
        let mut idx = 0;
        // first pass
        for _i in 0..cnt{
            self.first_pass(idx);
            idx += glen;
        }
        let mut results: Vec<Box<dyn Fn(&Vec<f32>) -> f32>> = Vec::with_capacity(cnt);
        // second pass
        idx = 0;
        for _i in 0..cnt{
            results.push(self.second_pass(idx));
            idx += glen;
        }
        Box::new(move |args| {
            let mut v :Vec<f32> = Vec::with_capacity(cnt);
            for i in 0..cnt{
                v.push((results[i])(args))
            }
            LF.evaluate(v)
        })
    }
    pub fn pass<'a>(&'a self, evaluator: &'a impl FitnessEvaluator) -> f32{
        let func = self.translate();
        let f = evaluator.evaluate(func);
        self.fitness.set(f);
        f
    }
    pub fn equity<'a>(&'a self, train:bool, evaluator: &'a impl FitnessEvaluator) -> Vec<f32>{
        let func = self.translate();
        let f = evaluator.equity(train,func);
        f
    }
    fn first_pass(&self, p:usize) {
        let mut pos = p;
        let mut first_arg_position:usize = p+1;
        loop {
            let current = &self.codons[pos];
            current.set_first_arg_position(first_arg_position);
            first_arg_position += usize::from(current.get_arity());
            pos += 1;
            if pos == first_arg_position {
                break;
            }
        };
    }
    fn second_pass<'a>(&'a self, pos: usize) -> Box<dyn Fn(&Vec<f32>) -> f32 + 'a> {
        let c = &self.codons[pos];
        return match c {
            Codon::Terminal(ref _t) => Box::new(move |args| c.evaluate(0.0, 0.0, args)),
            _ => {
                if c.get_arity() == 1{
                    Box::new(move |args|
                        c.evaluate(self.second_pass(c.get_first_arg_position())(args),
                            0.0, args))
                }
                else{
                    Box::new(move |args|
                        c.evaluate(self.second_pass(c.get_first_arg_position())(args),
                            self.second_pass(c.get_first_arg_position() +1)(args), args))
                }
            }
        };
    }
}