use num_bigint::{BigUint};

// TODO: Revisar o método e documentar 
pub fn miller_rabin_tester(num: BigUint) -> bool {
    let power_tester = BigUint::new(vec![1]);
    let mut s = 0;
    let mut d = &num.clone() - BigUint::new(vec![1]);

    while &d & &power_tester == BigUint::default() {
        s += 1;
        d >>= 1;
    }

    println!("s = {}\nd = {}\nnum = {}", s, d, num);

    let mut a = BigUint::new(vec![3]);

    while a < num {
        if a.modpow( &d, &num) == BigUint::new(vec![1]) {
            return true;
        } else {
            a += BigUint::new(vec![1]);
        }
    }

    let mut r: u32 = 0;
    a = BigUint::new(vec![3]);

    while a < num {
        while r < s {
            if a.modpow(&(&d * BigUint::new(vec![2]).pow(r)) , &num) == &num - BigUint::new(vec![1]) {
                return true;
            } else {
                r += 1;
            }
        }
    }
    
    false
}
