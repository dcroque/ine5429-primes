use ine5429_primes::rand_gen::*;
use num_bigint::{BigUint, ToBigUint};

fn main() {
    //TODO: Achar raizes primitivas de 2^4253-1, como candidato a bom argumento de multiplicador
    let mlcg_mult: BigUint = 16807.to_biguint().unwrap();
    let mut mlcg_gen: Mlcg = Mlcg::new_mersene(mlcg_mult, 4253);

    println!("{:b}\n", mlcg_gen.rand_sized(40));
}
