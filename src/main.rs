extern crate blake2;

use std::env::args;
use blake2::Blake2bVar;
use blake2::digest::{Update, VariableOutput};

fn main() {
    let mut hasher = Blake2bVar::new(16).unwrap();
    let mut buf = [0u8; 16];
    let mut newpass = String::new();
    let charset: Vec<char> = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_-+={[}]|:;'<,>.?")
        .chars()
        .collect();

    let arguments:Vec<String> = args().collect();
    
    
    if arguments.len() < 2{ //Check for enough arguments
        println!("Not enough arguments")
    } else{
        for i in arguments{
            hasher.update(i.as_bytes()); //update hasher with arguments
        }
        hasher.finalize_variable(&mut buf).unwrap();
        for i in buf{
            let characternumber = ((i as f64/255.0)*charset.len() as f64) as usize; //calculate character to select
            newpass.push(charset[characternumber]); //add selected character to password 
        }
        println!("{}", newpass);
    }
}
