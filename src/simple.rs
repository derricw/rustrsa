///
/// maths.rs
/// 
/// Math needed for a minimal RSA implementation.
/// 
/// TODO: maybe use https://aatch.github.io/ramp/ramp/int/struct.Int.html for
/// arbitrary large int sizes.
/// 

extern crate rand;
extern crate mod_exp;

use rand::prelude::*;
use mod_exp::mod_exp;

// fn main() {
//     println!("Hello, world!");
//     let x = simple_is_prime(10);
//     println!("{}", x);
//     let mut data: [bool; 100] = [false; 100];
//     prime_sieve(&mut data);

//     for i in 0..100 {
//         println!("{}: {}", i, data[i]);
//     }

//     println!("{}", rabin_miller(2003, 5));

// }


fn simple_is_prime(num: u128) -> bool {
    if num < 2 {
        false
    } else {
        //let sqrt = num.sqrt();  // ints don't have sqrt method
        let sqrt = (num as f64).sqrt() as u128;
        for i in 2..(sqrt+1) {
            if num % i == 0 {
                return false  // need return here
            }
        }
        true
    }
}


/// sieve of aratosthenesetenenses
fn prime_sieve(data: &mut [bool]) {
    // fills array with 1's
    for item in &mut data[..] {
        *item = true;
    }
    data[0] = false;
    data[1] = false;
    
    let size = data.len();
    let sqrt = (size as f64).sqrt() as usize;
    let mut pointer: usize;

    // sieve out 
    for i in 2..(sqrt+1) {
        pointer = i * 2;
        while pointer < size {
            data[pointer] = false;
            pointer = pointer + i as usize;
        }
    }
}

/// Primality test using Rabin Miller Algorithm
/// k = 5 is sufficiently accurate for most purposes
/// returns true if n is prime
fn rabin_miller(n: u64, k: u8) -> bool {
    if n % 2 == 0 { return false };
    let mut d = n-1;
    let mut r = 0;
    // find r and d
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }
    // witness loop
    let mut rng = thread_rng();
    for _attempts in 0..k {
        let a = rng.gen_range(2, n-1);
        let mut x = mod_exp(a, d, n);
        if x != 1 {
            let mut i = 0;
            while x != (n-1) {
                if i == (r-1) {
                    return false;
                } else {
                    x = x.pow(2) % n;
                    i += 1;
                    println!("x: {} i: {}", x, i)
                }
            }
        }
    }
    // we didn't find a witness after 
    // k attempts so it is probably prime
    true
}


fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    rabin_miller(n, 5)
}


fn find_random_prime(start: u64, stop: u64) -> u64 {
    let mut rng = thread_rng();
    loop {
        let e = rng.gen_range(start, stop);
        if is_prime(e) {
            return e;
        }
    }
}


fn multinv(modulus: u64, val: u64) -> u64 {
    let mut x = 0;
    let mut lastx = 1;
    let mut a = modulus;
    let mut b = val;
    let mut q;

    while b >= 1 {
        let (j, k, l) = (b, a/b, a%b);
        a = j;
        q = k;
        b = l;
        let (m, n) = (lastx - q * x, x);
        x = m;
        lastx = n;
    }
    let mut result = (1 - lastx * modulus) / val;
    result
}