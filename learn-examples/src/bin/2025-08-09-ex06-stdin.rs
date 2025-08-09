
//-- Modulo io de la biblioteca estándard
use std::io;

fn main() 
{
    //-- Descriptor de acceso a la entrada estándar
    let stdin = io::stdin();

    //-- Buffer donde guardar la cadena
    let mut buffer = String::new();

    println!("Introduce una cadena: ");

    //-- Leer cadena
    //-- Terminar si hay error
    stdin.read_line(&mut buffer)
        .expect("Error en la lectura");

    println!("Cadena: {}", buffer);
}
