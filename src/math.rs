/// math.rs
/// 
/// Math required for public key cryptography.
/// Uses external `ramp` crate for arbitrary integer sizes.
/// 
use ramp::RandomInt;
use ramp::Int;

/// simple prime test using arbitrary length `Int` structures
/// unfortuately its a little uglier than with regular int types
/// It works though, so I'll give it another pass after I've gotten
/// better at Rust
pub fn simple_is_prime(num: &Int) -> bool {
    if num < &2 {
        false
    } else {
        let s;
        let nsqr = Int::clone(num);      // sqrt_rem consumes its Int
                                         // so we have to use a clone
        match nsqr.sqrt_rem() {          // sqrt_rem returns an Option so we have to
            Some(x) => {                 // use match operator
                let (sr, _rr): (Int, Int) = x;
                s = Int::clone(&sr);
            },
            None => panic!("{} has no square root", num),
        }
        // i want to use a for loop here for some reason
        // that doesn't work with `Int`s
        let mut i = Int::from(2);
        let mut max = Int::clone(&s);
        max = max + 1;

        while &i < &max {
            if num % &i == 0 {
                return false
            }
            i = i + 1;
        }
        true
    }
}

/// Rabin-Miller primality testing
/// probabilistic, use higher K to reduce false negatives
/// at the cost of speed
/// TODO: use the deterministic version
/// 
pub fn rabin_miller(n: &Int, k: u8) -> bool {
    if n % 2 == 0 { return false };
    let mut d = n-1;
    let mut r = Int::zero();
    // find r and d
    while &d % 2 == 0 {
        d /= 2;
        r += 1;
    }
    // println!("d: {}\nr: {}", d, r);
    // witness loop
    let mut rng = rand::thread_rng();
    let one = Int::one();
    let two = Int::from(2);
    for _attempts in 0..k {
        let max = Int::clone(&n) - 1;
        let a = rng.gen_int_range(&two, &max);
        let mut x = a.pow_mod(&d, &n);
        // println!("a: {}\nx: {}", a, x);
        if &x != &one {
            let mut i = Int::zero();
            while &x != &(n-1) {
                if &i == &(Int::clone(&r)-1) {
                    return false;
                } else {
                    x = x.pow_mod(&two, &n);
                    i += 1;
                    // println!("x: {} i: {}", x, i);
                }
            }
        }
    }
    // we didn't find a witness after 
    // k attempts so it is probably prime
    true
}


/// Fancy prime checker that checks small
///     numbers first then tries R-M
pub fn is_prime(n: &Int) -> bool {
    let mut max: u32 = 1000;
    if n < &Int::from(max) {
        max = u32::from(n) - 1;
    }
    for i in 2..max {
        if n % &Int::from(i) == 0 {
            return false;
        }
    }
    return rabin_miller(n, 5);
}



pub fn find_random_prime(bits: usize) -> Int {
    let mut rng = rand::thread_rng();
    loop {
        let n = rng.gen_uint(bits);
        if is_prime(&n) {
            return n
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    //use ramp::RandomInt;
    
    #[test]
    fn simple_is_prime_works() {
        //let mut rng = rand::thread_rng();
        //let size: usize = 2048;
        //let num = rng.gen_uint(size);
        let prime = Int::from(67);
        let composite = Int::from(66);
        
        assert!(simple_is_prime(&prime));
        assert!(!simple_is_prime(&composite));
    }

    #[test]
    fn rabin_miller_works() {
        // let mut rng = rand::thread_rng();
        // let size: usize = 2048;
        // let num = rng.gen_uint(size);
        
        assert!(rabin_miller(&Int::from(67), 5));
        assert!(!rabin_miller(&Int::from(66), 5));

        // lets try a mersenne prime with 386 digits
        let bignum = Int::from(2).pow(1279) - 1;
        // println!("{}", bignum);
        assert!(rabin_miller(&bignum, 5));

    }

    #[test]
    fn is_prime_works() {
        assert!(is_prime(&Int::from(67)));
        assert!(!is_prime(&Int::from(66)));

        // lets try a mersenne prime with 386 digits
        let bignum = Int::from(2).pow(1279) - 1;
        // println!("{}", bignum);
        assert!(is_prime(&bignum));
    }

    #[test]
    fn find_random_prime_works() {
        let p: Int = find_random_prime(512);
        assert!(is_prime(&p));
        // println!("{}", p)
    }
}