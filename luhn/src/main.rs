use std::io::{stdin};
use luhn::is_valid;

mod lib;

fn main() {

    let mut code = String::new();
    println!("Inserire una carta di credito : ");
    stdin().read_line(&mut code).unwrap();

    if  is_valid(&code) {
        println!("Carta di credito valida!");
    } else {
        println!("Carta di credito non valida!");
    }
}