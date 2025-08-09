use std::{fs::File, io::Write};

fn main() 
{
    //-- Nombre del archivo binario
    const FILE_NAME:&str = "test.bin";

    //-- Crear fichero
    let mut file = File::create(FILE_NAME)
        .expect("Error al crear el archivo!!");

    //-- Datos a escribir
    let data = [0x01, 0x02, 0x03, 0x04];

    //-- Escribir!
    file.write(&data).expect("Error en la escritura");

}
