use std::io::BufReader;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::prelude::*;
use std::io::Error;

mod utiles;

#[derive(Clone)]
#[derive(Debug)]
struct Inventario {
    reactivo: String,
    stock:String,
    uso:String,
    ubicacion:String,
    codigo:String
}

//funcion buscar (find tipico del codigo anterior)
fn find() {
    let mut vector = Vec::new();
    let mut num_clave = 0;
    if let Ok(lines) = read_lines("./inventario.txt") {
    let mut nuevo_inventario = inicializar_struct();
    let mut contador_filas = 0;
    let mut palabra_clave = String::new();
    //este numero se usara por si quiere o no hacer un append al final del archivo
    // 0 = no hacer append | 1 = hacer append

    println!("Que desea hacer? Eliminar fila (E) | Modificar Stock-Uso (M) | Agregar nuevo reactivo (A)");
    let aux = utiles::ingreso_texto("OPCION".to_string());

    if aux == "e".to_string() || aux == "m".to_string() {
        palabra_clave = utiles::ingreso_texto("CODIGO DEL REACTIVO".to_string());
    }
        //RECORRE FILA 
        for line in lines {

            if let Ok(ip) = line {
                let ip_copy = ip.clone();
                let split = ip_copy.split(",");

                let mut contador_columnas = 0;

                //RECORRE COLUMNA
                for elemento in split {
                    match contador_columnas {
                        0 => nuevo_inventario.reactivo = elemento.to_string().trim().to_lowercase(),
                        1 => nuevo_inventario.stock = elemento.to_string().trim().to_lowercase(),
                        2 => nuevo_inventario.uso = elemento.to_string().trim().to_lowercase(),
                        3 => nuevo_inventario.ubicacion = elemento.to_string().trim().to_lowercase(),
                        4 => nuevo_inventario.codigo = elemento.to_string().trim().to_lowercase(),
                        
                        _ => (),

                    }
                    contador_columnas += 1;
                }   
            }

                //ESTO ES PARA EXCLUIR
                if aux == "e".to_string(){
                    if nuevo_inventario.codigo != palabra_clave && vector.len() < 6{
                        vector.push(nuevo_inventario.clone());
                    }
                    println!("{:?}\n\n",vector);

                    num_clave = 0;
                }

                //MODIFICAR STOCK Y USO DEL REACTIVO DE LA PALABRA CLAVE 
                if aux == "m".to_string(){

                    if nuevo_inventario.codigo == palabra_clave {
                        nuevo_inventario.stock = utiles::texto_numero("STOCK DEL REACTIVO".to_string()).to_string();
                        nuevo_inventario.uso = utiles::texto_numero("USO DEL REACTIVO".to_string()).to_string();
                    }

                    //SI NO EXISTE UN STOCK = 0 Y USO = 0, SE VA A PUSHEAR AL VECTOR
                    if verificador_stock_uso(&nuevo_inventario){
                        vector.push(nuevo_inventario.clone());
                    } 
                    println!("{:?}\n\n",vector);

                    num_clave = 0;
                }

                //REALIZAR EL APPEND
                if aux == "a".to_string(){
                    vector.push(nuevo_inventario.clone());

                    num_clave = 1;
                }
            contador_filas += 1;
        }
        // println!("{contador_filas}");
    }
    actualizar_archivo(vector,num_clave);
}

fn actualizar_archivo(vector: Vec<Inventario>,num_clave:i32) -> (){
    let mut contador = 0;
    let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .open("inventario.txt")
    .unwrap();

    println!("EL LARGO DEL ARREGLO ES {}",vector.len());
    for i in 1..vector.len(){
        let estructura = &vector[i];

        if contador == 0{
            let print_string = format!("{},{},{},{},{}",estructura.reactivo,estructura.stock,estructura.uso,
            estructura.ubicacion,estructura.codigo);
            file.write_all(print_string.as_bytes()).expect("NOFUNCIONO EL WRITE ALL");

        } else {

            let print_string = format!("\n{},{},{},{},{}",estructura.reactivo,estructura.stock,estructura.uso,
            estructura.ubicacion,estructura.codigo);
            file.write_all(print_string.as_bytes()).expect("NOFUNCIONO EL WRITE ALL");

        }
        contador += 1;

    }
    if num_clave == 1{

        append_archivo();

    }

}

fn append_archivo() {

    let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open("inventario.txt")
    .unwrap();


    println!("VAMOS A AGREGAR ALGO NUEVO YUPII ");

    let new_struct = Inventario{
        reactivo:utiles::ingreso_texto("REACTIVO".to_string()),
        stock:utiles::ingreso_texto("STOCK".to_string()),
        uso:utiles::ingreso_texto("USO".to_string()),
        ubicacion:utiles::ingreso_texto("UBICACION".to_string()),
        codigo:utiles::ingreso_texto("CODIGO".to_string())
    };

    let print_string = format!("\n{},{},{},{},{}",new_struct.reactivo,new_struct.stock,
    new_struct.uso,new_struct.ubicacion,new_struct.codigo);
    file.write_all(print_string.as_bytes()).expect("NOFUNCIONO EL WRITE ALL");


}

fn verificador_stock_uso(estructura:&Inventario) -> bool{

    if estructura.stock == 0.to_string() && estructura.uso == 0.to_string(){
        return false;
    } else {
        return true;
    }

}

fn inicializar_struct() -> Inventario{
    let inicializar: Inventario = Inventario {
        reactivo: String::new(),
        stock:String::new(),
        uso:String::new(),
        ubicacion: String::new(),
        codigo: String::new()    
    };

    return inicializar;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}


fn main(){
    find();

}