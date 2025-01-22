use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let d1 = rand::thread_rng().gen_range( 1..=6 );
    let d2 = rand::thread_rng().gen_range( 1..=6 );
    println!( "{d1} {d2}" );
}
