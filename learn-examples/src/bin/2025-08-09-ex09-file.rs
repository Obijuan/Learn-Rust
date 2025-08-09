use std::{fs::File, io::Read};

fn main() 
{
    //-- Nombre del archivo binario
    const FILE_NAME:&str = "test.bin";

    //-- Abrir fichero
    let mut file = File::open(FILE_NAME)
        .expect("Error al leer fichero...");    

    //-- Buffer donde leer el contenido del fichero
    let mut buffer: Vec<u8> = Vec::new();

    //-- Leer el archivo
    file.read_to_end(&mut buffer)
        .expect("Error en la lectura!");

    //-- Imprimir todos los valores que hay en el buffer
    println!("Contenido del fichero {}", FILE_NAME);
    for byte in buffer {
        print!("{} ", byte);
    }
    println!();

}

