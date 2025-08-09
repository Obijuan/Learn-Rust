fn main() 
{
    //-- Vector de bytes vacío
    let mut v: Vec<u8> = Vec::new();

    //-- Meter elementos dinámicamente
    v.push(0x01);
    v.push(0x02);
    v.push(0x03);

    //-- Imprimir todos los elementos
    for e in v {
        println!("{}", e);
    }
}
