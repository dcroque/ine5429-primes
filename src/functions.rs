use std::time::Instant;

use log::info;
use num_bigint::BigUint;

use crate::prime_test::*;
use crate::rand_gen::*;

pub fn find_fermat(size: u64, seed: &BigUint) -> BigUint {
    info!("Finding prime with {} bits with Fermat method", size);
    let now = Instant::now();
    let mut counter = 0;
    let mut mlcg_gen: Mlcg = Mlcg::new_std(size, seed);

    loop {
        let num = mlcg_gen.rand();
        counter += 1;
        if fermat_tester(&num, seed) {
            let elapsed_time = now.elapsed().as_secs_f32();
            info!(
                "Found prime! Search stats: {} checks in {:.4}s ({:.4}s/check avg)",
                counter,
                elapsed_time,
                elapsed_time / counter as f32
            );
            return num;
        }
    }
}

pub fn find_miller_rabin(size: u64, seed: &BigUint) -> BigUint {
    info!("Finding prime with {} bits with Miller_rabin method", size);
    let now = Instant::now();
    let mut counter = 0;
    let mut mlcg_gen: Mlcg = Mlcg::new_std(size, seed);

    loop {
        let num = mlcg_gen.rand();
        counter += 1;
        if miller_rabin_tester(&num, seed) {
            let elapsed_time = now.elapsed().as_secs_f32();
            info!(
                "Found prime! Search stats: {} checks in {:.4}s ({:.4}s/check avg)",
                counter,
                elapsed_time,
                elapsed_time / counter as f32
            );
            return num;
        }
    }
}
