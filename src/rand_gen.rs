use num_bigint::{BigUint, ToBigUint};
use std::ops::{BitAnd, BitOr};
use std::time::{SystemTime, UNIX_EPOCH};

/// Multiplicative linear congruential generator, também conhecido como Park-Miller RNG. Calcula novos valores por meio da fórmula s = mu*s % mo, sendo _s_ o ultimo valor gerado (ou inicialmente a semente), _mu_ um multiplicador e _mo_ o modulo do gerador. Recomenda-se que _mu_ e _mo_ sejam pelo menos coprimos.
pub struct Mlcg {
    /// Corresponde ao último numero gerado pela estrutura.
    state: BigUint,
    /// Semente do gerador
    seed: BigUint,
    /// Fator que multiplica o valor atual na geração do próximo.
    mult_factor: BigUint,
    /// Valor pelo qual é realizada a divisão modular na geração do próximo valor.
    mod_factor: BigUint,
    /// Tamanho dos valores gerados
    size: u64,
}

impl Mlcg {
    /// Constrói um novo MLCG com multiplicador = _mu_, modulo = _mo_, semente gerada com base no horário local e tamanho de _size_ bits.
    pub fn new(mu: BigUint, mo: BigUint, size: u64) -> Self {
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
            size,
        }
    }

    /// Constrói um novo MLCG com multiplicador = _mu_, modulo = _mo_, semente = _s_ e tamanho de _size_ bits.
    pub fn new_from_seed(mu: BigUint, mo: BigUint, size: u64, s: &BigUint) -> Self {
        Mlcg {
            state: Mlcg::warm_up(&s, &mu, &mo),
            seed: s.clone(),
            mult_factor: mu,
            mod_factor: mo,
            size,
        }
    }

    /// Constrói um novo MLCG com multiplicador = _mu_, modulo = 2^_mer_-1, semente gerada com base no horário local e tamanho de _size_ bits.
    pub fn new_mersene(mu: BigUint, mer: u32, size: u64) -> Self {
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
            size,
        }
    }

    /// Constrói um novo MLCG com multiplicador = _mu_, modulo = 2^_mer_-1, semente = _s_ e tamanho de _size_ bits.
    pub fn new_mersene_from_seed(mu: BigUint, mer: u32, size: u64, s: &BigUint) -> Self {
        let mo = BigUint::pow(&2.to_biguint().unwrap(), mer) - 1.to_biguint().unwrap();

        Mlcg {
            state: Mlcg::warm_up(&s, &mu, &mo),
            seed: s.clone(),
            mult_factor: mu,
            mod_factor: mo,
            size,
        }
    }

    /// Retorna o último valor pseudo aleatório gerado pela estrutura.
    pub fn value(&self) -> BigUint {
        self.state.clone()
    }

    /// Retorna a semente da estrutura.
    pub fn seed(&self) -> BigUint {
        self.seed.clone()
    }

    /// Retorna o pŕoximo valor pseudo aleatório calculado pela estrutura.
    fn calculate_next(&mut self) -> BigUint {
        self.state = (&self.state * &self.mult_factor) % &self.mod_factor;
        self.value()
    }

    /// Retorna o pŕoximo valor pseudo aleatório gerado pela estrutura com um tamanho específicado de bits.
    pub fn rand(&mut self) -> BigUint {
        let temp = &self.calculate_next();
        let msb: BigUint = 1.to_biguint().unwrap() << (self.size - 1);
        let ones: BigUint = (1.to_biguint().unwrap() << self.size) - 1.to_biguint().unwrap();
        temp.bitand(ones).bitor(msb)
    }

    /// Realiza um aquecimento na semente para evitar que os valores iniciais correspondam à semente e/ou exponham o multiplicador utilizado.
    fn warm_up(s: &BigUint, mu: &BigUint, mo: &BigUint) -> BigUint {
        if (s << 1) < *mo {
            Mlcg::warm_up(&(s * mu % mo), mu, mo)
        } else {
            s.clone()
        }
    }
}

pub struct LaggedFibonacci<T>
where
    T: Fn(BigUint, BigUint) -> BigUint,
{
    /// Operação realizada entre os valores n-j e n-k, como por exemplo Mul::mul, Add::add ou BitXor::bitxor
    operation: T,
    /// Semente do gerador
    seed: BigUint,
    /// Histórico de valores gerados/inseridos, para geração de novos
    states: Vec<BigUint>,
    /// Indicação de antecessor mais recente para utilizar na geração do próximo elemento
    ele_j: usize,
    /// Indicação de antecessor menos recente para utilizar na geração do próximo elemento
    ele_k: usize,
    /// Valor pelo qual é realizada a divisão modular na geração do próximo valor.
    mod_factor: BigUint,
    /// Tamanho dos valores gerados
    size: u64,
}

impl<T> LaggedFibonacci<T>
where
    T: Fn(BigUint, BigUint) -> BigUint,
{
    /// Constrói um novo Lagged-Fibonacci RNG com módulo _mo_ que realiza a operação _op_ entre os elementos n-_j_ e n-_k_, com lista inicial de elementos _elements_.
    pub fn new(elements: Vec<BigUint>, j: u16, k: u16, mo: BigUint, op: T, size: u64) -> Self {
        let seed: BigUint = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_biguint()
            .unwrap();

        let mut temp = LaggedFibonacci {
            operation: op,
            seed,
            states: elements,
            ele_j: j.into(),
            ele_k: k.into(),
            mod_factor: mo,
            size,
        };
        temp.check_initialization();
        temp
    }

    /// Constrói um novo Lagged-Fibonacci RNG com módulo _mo_ que realiza a operação _op_ entre os elementos n-_j_ e n-_k_.
    pub fn new_from_seed(
        elements: Vec<BigUint>,
        j: u16,
        k: u16,
        mo: BigUint,
        op: T,
        size: u64,
        s: &BigUint,
    ) -> Self {
        let mut temp = LaggedFibonacci {
            operation: op,
            seed: s.clone(),
            states: elements,
            ele_j: j.into(),
            ele_k: k.into(),
            mod_factor: mo,
            size,
        };
        temp.check_initialization();
        temp
    }

    /// Checa e corrige problemas referentes aos valores iniciais fornecidos para a inicialização da estrutura
    fn check_initialization(&mut self) {
        let j_k_pair = (self.ele_j, self.ele_k);

        match j_k_pair {
            (j, k) if j > k => {
                let temp = self.ele_j;

                self.ele_j = self.ele_k;
                self.ele_k = temp;
            },
            (j, k) if j == k => {
                self.ele_j -= 1;
            },
            (_, _) => ()
        }

        let k_len_pair = (self.ele_k, self.states.len());

        match k_len_pair {
            (k, l) if l > k => {
                self.states.truncate(self.ele_k);
            },
            (k, l) if l < k => {
                let mut temp = Mlcg::new_mersene_from_seed(
                    16087.to_biguint().unwrap(),
                    31,
                    self.size,
                    self.seed(),
                );
                while self.states.len() < self.ele_k {
                    self.states.insert(0, temp.rand() | 1.to_biguint().unwrap())
                }
            },
            (_, _) => ()
        }
    }

    /// Retorna o último valor pseudo aleatório gerado pela estrutura.
    pub fn value(&self) -> BigUint {
        self.states.last().unwrap().clone()
    }

    /// Retorna a semente da estrutura.
    pub fn seed(&self) -> &BigUint {
        &self.seed
    }

    /// Retorna o pŕoximo valor pseudo aleatório gerado pela estrutura.
    fn calculate_next(&mut self) -> BigUint {
        let ele_j: BigUint = self.states[self.states.len() - self.ele_j].clone();
        let ele_k: BigUint = self.states.remove(0);
        self.states
            .push((self.operation)(ele_j, ele_k) % &self.mod_factor);
        self.value()
    }

    /// Retorna o pŕoximo valor pseudo aleatório gerado pela estrutura com um tamanho específicado de bits.
    pub fn rand(&mut self) -> BigUint {
        let temp = &self.calculate_next();
        let msb: BigUint = 1.to_biguint().unwrap() << (self.size - 1);
        let ones: BigUint = (1.to_biguint().unwrap() << self.size) - 1.to_biguint().unwrap();
        temp.bitand(ones).bitor(msb)
    }
}
