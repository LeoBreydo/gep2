use rand::prelude::ThreadRng;
use rand::Rng;
use crate::codons::Codon;
use crate::functions::{FN_NUM, Function, REGISTRY};
use crate::linking_function::LF;
use crate::state_functions::{Collector, Delay, Diff, SFN_NUM};
use crate::terminal::Terminal;

pub struct Chromosome{
    head_size:usize,
    nbr_of_genes:usize,

    pub codons: Vec<Codon>,
    pub fitness: f32,
}
impl Chromosome{
    pub fn new(mut rng: &mut ThreadRng, mut geneNbr:usize,  mut headLength: usize, mut varNbr:usize) -> Self{
        if geneNbr < 1 {geneNbr = 1}
        if headLength < 1 {headLength = 1}
        if varNbr < 1 {varNbr = 1}

        let gl = 2*headLength + 1;
        let mut codons: Vec<Codon> = Vec::with_capacity(geneNbr*gl);
        Self::initialize_codons(&mut rng, &mut codons, geneNbr, headLength, varNbr);
        Chromosome { codons, head_size:headLength, nbr_of_genes:geneNbr, fitness:0.0 }
    }
    fn initialize_codons(mut rng: &mut ThreadRng, mut arr: &mut Vec<Codon>, gn:usize, hl:usize, na:usize){
        let gl = 2*hl+1;
        for i in 0..gn{
            let start = i*gl;
            // first codon in gene must be non-terminal
            arr.push(Self::create_non_terminal(rng));
            for _j in start+1..start+hl{
                Self::push_head_codon(&mut rng, &mut arr, na);
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
                    _ => ret.push_str(c.get_symbol())
                }
                ret.push('|');
            }
            if i < self.nbr_of_genes-1 {ret.push_str(" //");}
        }
        ret
    }

    // genetic operations
    pub fn mutation(&mut self, mut rng: &mut ThreadRng, argsNbr:usize, mut codon_mutation_probability: f32) -> Chromosome{
        let glen = self.head_size*2+1;
        // codons for mutated chromosome
        let mut codons: Vec<Codon> = Vec::with_capacity(self.codons.len());
        for i in 0..self.nbr_of_genes {
            let start = i * glen;
            //mutate first head position (must be non-terminal)
            if rng.gen_range(0.0..1.0) < codon_mutation_probability{
                codons[i] = Self::create_non_terminal(rng);
            }
            else{
                codons[i] = self.codons[i].clone();
            }
            // mutate rest of head
            for j in start+1..start + self.head_size {
                if rng.gen_range(0.0..1.0) < codon_mutation_probability{
                    let r = rng.gen::<usize>() % 10;
                    if r < 2 {
                        codons[j] = (Codon::Terminal(Terminal::new(rng.gen::<usize>() % argsNbr)));
                    } else {
                        codons[j] = Self::create_non_terminal(rng);
                    }
                }
                else{
                    codons[j] = self.codons[j].clone();
                }
            }
            //mutate tail
            for j in start + self.head_size..start + glen {
                if rng.gen_range(0.0..1.0) < codon_mutation_probability{
                    self.codons[j] = (Codon::Terminal(Terminal::new(rng.gen::<usize>() % argsNbr)));
                }
                else{
                    codons[j] = self.codons[j].clone();
                }
            }
        }
        Chromosome { codons, head_size:self.head_size, nbr_of_genes:self.nbr_of_genes, fitness: 0.0 }
    }
    // pub fn root_transposition(&mut self, rng: &mut ThreadRng, transposition_probability : f32) {
    //     let test = rng.gen_range(0.0..1.0);
    //     if test >= transposition_probability {return;}
    //     let len = self.codons.len();
    //     let mut ip = if rng.gen_range(0.0..1.0) < 0.5 {0}else{len/2};
    //     let mut start = rng.gen::<usize>() % len;
    //     let mut end = start + rng.gen::<usize>() % (len);
    //     if end >= len {end = len-1;}
    //     // try to find first occurrence of a function in a given transposon
    //     // new gene must start with a function
    //     loop{
    //         match self.codons[start]{
    //             Codon::Function(ref _f) => break,
    //             _ => start += 1
    //         }
    //         // no functions found -> no transposition
    //         if start > end {return;}
    //     };
    //     // max transposon length equals length of head
    //     let cnt = end-start+1;
    //     if cnt > self.hl {end = start+self.hl-1;}
    //     // actual transposition here
    //     for i in start..end+1{
    //         match self.codons[i] {
    //             Codon::Function(ref f) => {
    //                 self.codons[ip] = Codon::Function(Function::new(f.fd));
    //             },
    //             Codon::Terminal(ref t) => {
    //                 self.codons[ip] = Codon::Terminal(Terminal::new(t.i));
    //             }
    //         }
    //         ip += 1;
    //     }
    // }
    // pub fn two_points_crossover(chrs: &mut Vec<TwoGeneChromosome>, i1: usize, i2: usize, rng: &mut ThreadRng) {
    //     let len = chrs[0].codons.len();
    //     let point1 = rng.gen::<usize>() % len;
    //     let mut point2:usize;
    //     loop{
    //         point2 = rng.gen::<usize>() % len;
    //         if point1 != point2 {break}
    //     }
    //     let (start,end) = if point1 < point2 {(point1,point2)} else {(point2,point1)};
    //     // dbg!(start);
    //     // dbg!(end);
    //     let (a, b) = chrs.split_at_mut(i2);
    //     let mut p1 = &mut a[i1];
    //     let mut p2 = &mut b[0];
    //     for i in start..end+1{
    //         let (x, y) = (p2.codons[i].clone(), p1.codons[i].clone());
    //         p1.codons[i] = x;
    //         p2.codons[i] = y;
    //     }
    // }
}