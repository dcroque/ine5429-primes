use std::env;
use env_logger::Env;

pub fn init() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).try_init().unwrap()
}

pub fn get_arg() -> u64 {
    let args: Vec<String> = env::args().collect();
    let prime_test: u64 = args[1].parse().unwrap();
    prime_test
}