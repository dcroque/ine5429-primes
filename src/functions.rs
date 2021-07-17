use std::time::Instant;

use log::info;
use num_bigint::{BigUint, ToBigUint};

use crate::prime_test::*;
use crate::rand_gen::*;

pub fn find_fermat(size: u64) -> BigUint {
    info!("Finding prime with {} bits with Fermat method", size);
    let now = Instant::now();
    let mlcg_mult: BigUint = 16807.to_biguint().unwrap();
    //TODO: Achar raizes primitivas de 2^4253-1, como candidato a bom argumento de multiplicador
    let mut mlcg_gen: Mlcg = Mlcg::new_mersene(mlcg_mult, 4253, size);

    loop {
        let num = mlcg_gen.rand();
        if fermat_tester(&num) {
            info!("Found prime! Search time: {:.4}s", now.elapsed().as_secs_f32());
            return num;
        }
    }
}
