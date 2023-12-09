use std::io::BufReader;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::prelude::*;

pub fn texto_numero(campo:String) -> i32 {
    loop {
        println!("Ingrese un número para el/la {}: ",campo);
        let mut numero = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut numero).unwrap();
        let numero: i32 = match numero.trim().parse(){
            Ok(numero) => numero,
            Err(_) => {
                println!("Error, no es un número");
                continue;
            },
        };
        return numero;
    }
}


pub fn ingreso_texto(campo: String) -> String {

    println!("Ingrese {}", campo);
    let mut texto = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut texto).unwrap();
    return texto.trim().to_lowercase();

}