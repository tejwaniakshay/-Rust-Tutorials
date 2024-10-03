
extern crate rand;

use rand::Rng;
 
fn main() {
    println!("Random number generation ");
    let rand_no = rand::thread_rng().gen_range(1..10);
    println!("{}",rand_no);
}
