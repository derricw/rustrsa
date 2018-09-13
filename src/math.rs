//extern crate ramp;
//use self::ramp::Int;
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


