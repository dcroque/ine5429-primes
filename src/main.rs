use std::env;

use ine5429_primes::prime_test::*;
use ine5429_primes::rand_gen::*;

use num_bigint::{BigUint, ToBigUint};

fn main() {
    let args: Vec<String> = env::args().collect();
    let prime_test: u32 = args[1].parse().unwrap();

    //TODO: Achar raizes primitivas de 2^4253-1, como candidato a bom argumento de multiplicador
    let mlcg_mult: BigUint = 16807.to_biguint().unwrap();
    let mut _mlcg_gen: Mlcg = Mlcg::new_mersene(mlcg_mult, 4253, 40);

    let ini_vec = Vec::new();
    let lf_mod = 2.to_biguint().unwrap().pow(4253);
    let mut _lf_gen = LaggedFibonacci::new(ini_vec, 418, 1279, lf_mod, std::ops::Mul::mul, 64);

    let mersene_3217 = 2.to_biguint().unwrap().pow(prime_test) - BigUint::new(vec![1]);

    match miller_rabin_tester(mersene_3217) {
        true => println!("PRIMO\n"),
        false => println!("NOT PRIMO\n"),
    }
}
