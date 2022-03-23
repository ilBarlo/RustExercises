mod capitalize;

use clap::Parser;
// use std::io::{stdin};
use crate::capitalize::capitalize;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Sentence to capitalize
    // #[clap(short, long)] // to use <cmd> -s,--sentence <SENTENCE> syntax
    sentence: String,
}

fn main() {
    
    /*
    let mut s = String::new();
    println!("Inserire una stringa : ");
    stdin().read_line(&mut s).unwrap();
    */

    let args = Args::parse();
    let s = capitalize(&args.sentence);
    println!("La stringa convertita è '{}'", s);
}

#[cfg(test)]
mod tests {
    use crate::capitalize;

    #[test]
    fn more_than_one_word() {
        assert_eq!(capitalize("prova con tante parole"), "Prova Con Tante Parole");
    }

    #[test]
    fn one_word() {
        assert_eq!(capitalize("parola"), "Parola");
    }

    #[test]
    fn accent() {
        assert_eq!(capitalize("è una prova"), "È Una Prova");
    }

    #[test]
    //#[ignore]
    fn empty_string() {
        assert_eq!(capitalize(""), "");
    }

    #[test]
    //#[ignore]
    fn more_space() {
        assert_eq!(capitalize("prova   con più  spazi"), "Prova   Con Più  Spazi");
    }
}