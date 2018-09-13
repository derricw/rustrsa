
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


// fn rabin_miller(n: &Int, k: u8) -> bool {
//     if n % 2 == 0 { return false };
//     let mut d = n-1;
//     let mut r = 0;
//     // find r and d
//     while d % 2 == 0 {
//         d /= 2;
//         r += 1;
//     }
//     // witness loop
//     let mut rng = rand::thread_rng();
//     for _attempts in 0..k {
//         let a = rng.gen_range(2, n-1);
//         let mut x = mod_exp(a, d, n);
//         if x != 1 {
//             let mut i = 0;
//             while x != (n-1) {
//                 if i == (r-1) {
//                     return false;
//                 } else {
//                     x = x.pow(2) % n;
//                     i += 1;
//                     println!("x: {} i: {}", x, i)
//                 }
//             }
//         }
//     }
//     // we didn't find a witness after 
//     // k attempts so it is probably prime
//     true
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    
    fn is_prime_works() {
        let mut rng = rand::thread_rng();
        let size: usize = 2048;
        let num = rng.gen_uint(size);
        let is_it_prime = simple_is_prime(&num);

        println!("{} is prime: {}", num, is_it_prime);
    }
}