use num_bigint::{BigUint, ToBigUint};

// TODO: Revisar o método e documentar

use crate::rand_gen::Mlcg;

/// Aplica a checagem de Miller-Rabin para determinar se o numero é primo
pub fn miller_rabin_tester(num: &BigUint, seed: &BigUint) -> bool {
    let mut gen = Mlcg::new_mersene_from_seed(16807.to_biguint().unwrap(), 31, 32, seed);
    for _ in 0..20 {
        if !miller_rabin_witness(num, gen.rand()) {
            return false;
        }
    }
    true
}

/// Checagem de Miller-Rabin para determinar se o número é um forte candidato a primo (75%)
fn miller_rabin_witness(num: &BigUint, wit: BigUint) -> bool {
    if basic_non_prime_check(&num) {
        return false;
    }
    
    let mut s = 0;
    let mut d = &num.clone() - BigUint::new(vec![1]);

    while &d & BigUint::new(vec![1]) == BigUint::default() {
        s += 1;
        d >>= 1;
    }

    let mut witmodpow = wit.modpow(&d, &num);

    if witmodpow == BigUint::new(vec![1]) {
        return true;
    }

    while s > 0 {
        if witmodpow == num - BigUint::new(vec![1]) {
            return true;
        } else {
            witmodpow = &witmodpow * &witmodpow % num;
            s -= 1
        }
    }
    false
}

/// Aplica a checagem de Fermat para determinar se o numero é primo
pub fn fermat_tester(num: &BigUint, seed: &BigUint) -> bool {
    let mut gen = Mlcg::new_mersene_from_seed(16807.to_biguint().unwrap(), 31, 32, seed);
    for _ in 0..8 {
        if !fermat_witness(num, gen.rand()) {
            return false;
        }
    }
    true
}

/// Checagem de Fermat para determinar se o número é um forte candidato a primo
fn fermat_witness(num: &BigUint, wit: BigUint) -> bool {
    if basic_non_prime_check(&num) {
        return false;
    }

    wit.modpow(&(num - BigUint::new(vec![1])), num) == BigUint::new(vec![1])
}

/// Checa se o número é par ou se num²-1 não é multiplo de 24, condições que impossibilitam a primalidade
fn basic_non_prime_check(num: &BigUint) -> bool {
    (num % BigUint::new(vec![2]) == BigUint::default())
        | ((num * num - BigUint::new(vec![1])) % 24.to_biguint().unwrap() != BigUint::default())
}
