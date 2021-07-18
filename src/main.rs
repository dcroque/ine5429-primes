use std::time::Instant;

use log::info;

use ine5429_primes::{environment, functions::*, rand_gen::*};

fn main() {
    let args = environment::init();

    info!("Arguments successfully parsed");

    match args.op {
        // Gerar números aleatórios
        true => {
            match args.method {
                // MLCG
                true => {
                    let now = Instant::now();
                    let mut mlcg_gen = Mlcg::new_std(args.size, &args.seed);
                    info!(
                        "MLCG initialization time: {:.4}s",
                        now.elapsed().as_secs_f64()
                    );
                    for i in 0..args.n {
                        info!("{}º: {}", i+1, mlcg_gen.rand());
                    }
                    let elapsed = now.elapsed().as_secs_f64();
                    info!(
                        "Total time for generating {} numbers: {:.4}ms ({:.4}ms/number avg)",
                        args.n,
                        elapsed * 1000 as f64,
                        elapsed * (1000 / args.n) as f64
                    );
                }
                // Lagged Fibonacci
                false => {
                    let now = Instant::now();
                    let mut lf_gen =
                        LaggedFibonacci::new_std(args.size, &args.seed, std::ops::Mul::mul);
                    info!(
                        "Lagged Fibonacci initialization time: {:.4}s",
                        now.elapsed().as_secs_f64()
                    );
                    for i in 0..args.n {
                        info!("{}º: {}", i+1, lf_gen.rand());
                    }
                    let elapsed = now.elapsed().as_secs_f64();
                    info!(
                        "Total time for generating {} numbers: {:.4}ms ({:.4}ms/number avg)",
                        args.n,
                        elapsed * 1000 as f64,
                        elapsed * (1000 / args.n) as f64
                    );
                }
            }
        }
        // Gerar primos
        false => {
            let now = Instant::now();
            let mut seed_gen = Mlcg::new_std(512, &args.seed);
            info!(
                "MLCG for seed generation initialization time: {:.4}s",
                now.elapsed().as_secs_f64()
            );
            match args.method {
                // Miller-Rabin
                true => {
                    for i in 0..args.n {
                        info!("{}º: {}", i+1, find_miller_rabin(args.size, &seed_gen.rand()));
                    }
                    let elapsed = now.elapsed().as_secs_f64();
                    info!(
                        "Total time for generating {} numbers: {:.4}s ({:.4}s/number avg)",
                        args.n,
                        elapsed,
                        elapsed / args.n as f64
                    )
                },
                // Fermat
                false => {
                    for i in 0..args.n {
                        info!("{}º: {}", i+1, find_fermat(args.size, &seed_gen.rand()));
                    }
                    let elapsed = now.elapsed().as_secs_f64();
                    info!(
                        "Total time for generating {} numbers: {:.4}s ({:.4}s/number avg)",
                        args.n,
                        elapsed,
                        elapsed / args.n as f64
                    )
                }
            }
        }
    }
}
