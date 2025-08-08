fn main() 
{
    //-- Definir una lista de 3 bytes
    let v: Vec<u8> = vec![0xff, 0xaa, 0x00];

    //-- Imprimir los bytes
    for byte in v {
        println!("Byte: {:#X}", byte);
    }

}