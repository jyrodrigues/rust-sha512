extern crate crypto;

use self::crypto::sha3::Sha3;
use crate::crypto::digest::Digest;
use std::env::args;

fn main() {
    /*
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    */

    let hash_input = args().nth(1).expect("A hash input is required");

    let mut hasher = Sha3::sha3_512();

    hasher.input_str(&hash_input);

    println!("{}", hasher.result_str());
}
