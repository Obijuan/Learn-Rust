fn main() 
{
    //-- Vector de 5 elementos inicializados a 0
    //-- [Valor; tamaño]
    let v: Vec<u8> = vec![0; 5];

    //-- Imprimir longitud del vector
    println!("Numero de bytes: {}", v.len());

    //-- Imprimir los bytes
    print!("Vector: [");
    for byte in v {
        print!("{:#X} ", byte);
    }
    println!("]");
}
