
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
    let status = stdin.read_line(&mut buffer);

    //-- Comprobar el resultado de la operación
    match status {
        Ok(bytes) => {
            println!("Bytes leidos: {}", bytes);
            println!("Cadena: {}", buffer);
        }
        Err(msg) => {
            println!("Error!: {}", msg);
        }
    }
}
