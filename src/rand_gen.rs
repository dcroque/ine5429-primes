use num_bigint::{BigUint, ToBigUint};
use std::ops::{BitAnd, BitOr};
use std::time::{SystemTime, UNIX_EPOCH};

// TODO: Documentar classe e implementações
pub struct Mlcg {
    state: BigUint,
    seed: BigUint,
    mult_factor: BigUint,
    mod_factor: BigUint,
}

impl Mlcg {
    pub fn new(mu: BigUint, mo: BigUint) -> Self {
        let seed: BigUint = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_biguint()
            .unwrap();

        Mlcg {
            state: Mlcg::warm_up(&seed, &mu, &mo),
            seed,
            mult_factor: mu,
            mod_factor: mo,
        }
    }

    pub fn new_from_seed(s: BigUint, mu: BigUint, mo: BigUint) -> Self {
        Mlcg {
            state: Mlcg::warm_up(&s, &mu, &mo),
            seed: s,
            mult_factor: mu,
            mod_factor: mo,
        }
    }

    pub fn new_mersene(mu: BigUint, mer: u32) -> Self {
        let seed: BigUint = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_biguint()
            .unwrap();
        let mo = BigUint::pow(&2.to_biguint().unwrap(), mer) - 1.to_biguint().unwrap();

        Mlcg {
            state: Mlcg::warm_up(&seed, &mu, &mo),
            seed,
            mult_factor: mu,
            mod_factor: mo,
        }
    }

    pub fn new_mersene_from_seed(s: BigUint, mu: BigUint, mer: u32) -> Self {
        let mo = BigUint::pow(&2.to_biguint().unwrap(), mer) - 1.to_biguint().unwrap();

        Mlcg {
            state: Mlcg::warm_up(&s, &mu, &mo),
            seed: s,
            mult_factor: mu,
            mod_factor: mo,
        }
    }

    pub fn value(&self) -> BigUint {
        self.state.clone()
    }

    pub fn seed(&self) -> BigUint {
        self.seed.clone()
    }

    pub fn rand(&mut self) -> BigUint {
        self.state = (&self.state * &self.mult_factor) % &self.mod_factor;
        self.value()
    }

    pub fn rand_sized(&mut self, size: u64) -> BigUint {
        let temp = &self.rand();
        let msb: BigUint = 1.to_biguint().unwrap() << (size - 1);
        let ones: BigUint = (1.to_biguint().unwrap() << size) - 1.to_biguint().unwrap();
        temp.bitand(ones).bitor(msb)
    }

    fn warm_up(s: &BigUint, mu: &BigUint, mo: &BigUint) -> BigUint {
        if (s << 1) < *mo {
            Mlcg::warm_up(&(s * mu % mo), mu, mo)
        } else {
            s.clone()
        }
    }
}
