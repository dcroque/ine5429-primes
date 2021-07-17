use std::process::exit;

use clap::{App, load_yaml};
use env_logger::Env;
use log::{error, info, warn};

/// Estrutura contendo os argumentos já tratados e testados
pub struct ParsedArgs {
    /// Operação: true para gerar números aleatórios, false para gerar primos 
    pub op: bool,
    /// Número de bits para os valores gerados
    pub size: u64, 
    /// Método: true para MLCG/Miller-Rabin, false para Lagged Fibonacci/Fermat (RNG/Checagem de primos)
    pub method: bool,
    /// Quantidade de números para gerar
    pub n: u64,
}

/// Inicialiaza o logging, recebe os argumentos de execução e testa se todos estão corretos
pub fn init() -> ParsedArgs {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).try_init().unwrap();

    let yaml = load_yaml!("cli.yaml");
    let args = App::from(yaml).get_matches();
    let mut parsedargs = ParsedArgs { 
        op: true, 
        size: 256, 
        method: true, 
        n: 1, 
    };

    let gen_flags = (args.is_present("rng"), args.is_present("prime"));

    match gen_flags {
        (r, p) if r & p => {
            error!("You can only run one operation per execution: remove -r/--rng or -p/--prime");
            exit(1)
        },
        (r, p) if !r & !p => {
            error!("You need to select one operation per execution: add -r/--rng or -p/--prime");
            exit(1)
        },
        (r, p) if r & !p => {
            match args.value_of("method") {
                Some(val) => {
                    match val {
                        "m" => info!("Random number generation chosen with MLCG method"),
                        "f" => {
                            info!("Random number generation chosen with Lagged Fibonacci method");
                            parsedargs.method = false;
                        },
                        _ => warn!("Wrong method string, only m or f are valid; using MLCG as default")
                    }
                },
                None => warn!("No method string set; using MLCG as default")
            }
        },
        _ => {
            match args.value_of("method") {
                Some(val) => {
                    match val {
                        "m" => info!("Random number generation chosen with Miller-Rabin method"),
                        "f" => {
                            info!("Random number generation chosen with Fermat method");
                            parsedargs.method = false
                        },
                        _ => warn!("Wrong method string, only m or f are valid; using Miller-Rabin as default")
                    }
                    parsedargs.op = false;
                },
                None => warn!("No method string set; using Miller-Rabin as default")
            }
        }
    }

    match args.value_of("size") {
        Some(val) => {
            match val.parse::<u64>() {
                Ok(num) => {
                    info!("Size set to {} bits", num);
                    parsedargs.size = num;
                },
                Err(_) => {
                    error!("Error trying to parse size");
                    exit(1)
                }
            }
        },
        None => warn!("No size given: default is 256 bits")
    }

    match args.value_of("quantity") {
        Some(val) => {
            match val.parse::<u64>() {
                Ok(num) => {
                    info!("Number of generations set to {}", num);
                    parsedargs.n = num;
                },
                Err(_) => {
                    error!("Error trying to parse quantity of numbers");
                    exit(1)
                }
            }
        },
        None => warn!("No number of operations given: default is 1")
    }

    parsedargs
}