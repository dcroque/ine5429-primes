use log::info;

use ine5429_primes::environment;

fn main() {
    let args = environment::init();

    info!("Arguments successfully parsed");

    match args.op {
        // Gerar números aleatórios
        true => {
            info!("Stuff")
        }
        // Gerar primos
        false => {
            info!("other sutff")
        }
    }
}
