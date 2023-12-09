


pub fn ingreso_texto(campo: String) -> String {

    println!("Ingrese {}", campo);
    let mut texto = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut texto).unwrap();
    return texto.trim().to_lowercase();

}