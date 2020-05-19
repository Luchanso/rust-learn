#![feature(test)]

mod prime;
mod user;

use crate::prime::erotosfen::calc_prime;

fn main() {
    let prime_number = calc_prime(user::get_input());

    println!("is prime: {:?}", prime_number);
}
