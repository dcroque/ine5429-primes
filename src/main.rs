use log::{info};

use ine5429_primes::environment;

fn main() {
    let args = environment::init();

    info!("\n\nArgs:\n\t{}\n\t{}\n\t{}\n\t{}", args.op, args.size, args.method, args.n)
}
