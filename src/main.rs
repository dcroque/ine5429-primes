use log::info;

use ine5429_primes::environment;
use ine5429_primes::functions::*;

fn main() {
    environment::init();

    let prime = find_fermat(environment::get_arg());
    info!("{}: {}", environment::get_arg(), prime);
}
