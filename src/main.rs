extern crate rustrsa;

use rustrsa::math;

fn main() {
    let num = 15;
    println!("{} is prime: {}", num, math::simple_is_prime(num));

}