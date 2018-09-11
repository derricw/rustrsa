extern crate ramp;

pub fn simple_is_prime(num: u128) -> bool {
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