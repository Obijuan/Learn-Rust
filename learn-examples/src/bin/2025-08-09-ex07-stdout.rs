
//-- Modulo io de la biblioteca estándard
use std::io::{self, Write};

fn main() 
{
    //-- Descriptor para la salida estándar
    let mut stdout = io::stdout();

    //-- Descriptor para la entrada estándar
    let stdin = io::stdin();

    //-- Imprimir cadena y hacer flush!
    print!("Introduce cadena: ");
    stdout.flush().expect("Error en flush!");

    //-- Leer cadena
    let mut cad= String::new();
    stdin.read_line(&mut cad)
        .expect("Error en la lectura");

    println!("Cadena: {}", cad);
}
