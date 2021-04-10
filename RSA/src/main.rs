use std::env;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::sync::mpsc;
use std::io::{BufRead, BufReader};
use std::fs::File;
extern crate num_bigint;
extern crate rand;
extern crate num;
extern crate ascii;
use ascii::ToAsciiChar;
use num_bigint::{RandBigInt, BigUint, RandomBits};
use rand::Rng;
use num::One; 

fn main() {
    let (tx, rx): (Sender<num_bigint::BigUint>, Receiver<num_bigint::BigUint>) = mpsc::channel();
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    // if args.len() != 5 {
    //     usage()
    // }
    let mut plaintext_file = BufReader::new(File::open(&args[1]).expect("open failed"));
    let mut buf = Vec::<u8>::new();
    let mut chars_to_encode = Vec::<u8>::new();
    while plaintext_file.read_until(b'\n', &mut buf).expect("read_until failed") != 0 {
        let s = String::from_utf8(buf).expect("from_utf8 failed");
        for c in s.chars() {
            println!("{}", c);
            chars_to_encode.push(c.to_ascii_char().unwrap().as_byte());
        }
        buf = s.into_bytes();
        buf.clear();
    }
    println!("Contents of xs:");
    for x in chars_to_encode.iter() {
        println!("> {}", x);
    }

    // let ciphertext_filename = args[2];
    // let decrptedtext_filename = args[3];
    let bitlength = args[2].parse::<usize>().unwrap();
    thread::spawn(move || {
        tx.send(generate_prime(bitlength)).unwrap();
    });
    let q = generate_prime(bitlength);
    let p = rx.recv().unwrap();

    
    println!("{}", p);
    println!("{}", q);

}

fn encrypt(byte : BigUint, n : BigUint, e : BigUint) -> BigUint {
    return byte.modpow(&e, &n);
}


fn generate_prime(bitlength : usize ) -> BigUint {
    
    let mut rng = rand::thread_rng();
    loop {
        let prime: BigUint = rng.sample(RandomBits::new(bitlength));
        if primality_test(&prime, 100) {  return prime; }
    }
}

fn primality_test(prime : &BigUint, n : i32) -> bool {
    let mut rng = rand::thread_rng();
    let one : &BigUint = &One::one();
    let p_one: BigUint = prime - one;
    for _x in 0..n {
        let a = rng.gen_biguint_range(one, &(p_one));
        if a.modpow(&p_one, prime) != *one {
            return false;
        }
    }
    return true;
}


fn usage() {
    println!("Usage: [plaintext filename] [ciphertext filename] [decryptedtext filename ] [bit-length]");
    panic!("Incorrect # of command line parameters")
}