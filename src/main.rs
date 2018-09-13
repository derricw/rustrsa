#![feature(extern_prelude)]
extern crate rustrsa;
extern crate ramp;

use ramp::RandomInt;
use rustrsa::math;

fn main() {

    let mut rng = rand::thread_rng();
    let size: usize = 2048;
    let num = rng.gen_uint(size);
    let is_it_prime = math::simple_is_prime(&num);

    println!("{} is prime: {}", num, is_it_prime);

}