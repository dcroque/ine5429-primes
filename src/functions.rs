use std::time::Instant;

use log::info;
use num_bigint::{BigUint, ToBigUint};

use crate::prime_test::*;
use crate::rand_gen::*;

pub fn find_fermat(size: u64, seed: &BigUint) -> BigUint {
    info!("Finding prime with {} bits with Fermat method", size);
    let now = Instant::now();
    let mut counter: f32 = 0.0;
    let mlcg_mult: BigUint = 16807.to_biguint().unwrap();
    //TODO: Achar raizes primitivas de 2^4253-1, como candidato a bom argumento de multiplicador
    let mut mlcg_gen: Mlcg = Mlcg::new_mersene_from_seed(mlcg_mult, 4253, size, seed);

    loop {
        let num = mlcg_gen.rand();
        counter += 1.0;
        if fermat_tester(&num, seed) {
            let elapsed_time = now.elapsed().as_secs_f32();
            info!(
                "Found prime! Search stats: {} checks in {:.4}s ({:.4}s/check avg)",
                counter,
                elapsed_time,
                elapsed_time / counter
            );
            return num;
        }
    }
}
