use ine5429_primes::rand_gen::*;
use num_bigint::{BigUint, ToBigUint};

fn main() {
    //TODO: Achar raizes primitivas de 2^4253-1, como candidato a bom argumento de multiplicador
    let mlcg_mult: BigUint = 16807.to_biguint().unwrap();
    let mut mlcg_gen: Mlcg = Mlcg::new_mersene(mlcg_mult, 4253, 40);

    println!("{:b}\n", mlcg_gen.rand());

    let ini_vec = Vec::new();
    let lf_mod = 2.to_biguint().unwrap().pow(4253);

    let mut lf_gen = LaggedFibonacci::new(ini_vec, 418, 1279, lf_mod, std::ops::Mul::mul, 64);

    for _ in 1 .. 30 {
        println!("{:b}\n", lf_gen.rand());
    }
}
