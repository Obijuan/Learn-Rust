fn main() 
{
    //-- Vector de bytes vacío
    let mut v: Vec<u8> = Vec::new();

    //-- Meter elementos dinámicamente
    v.push(0x01);
    v.push(0x02);
    v.push(0x03);

    for i in 0..5 {

        //-- Leer un elemento
        let e = v.get(i);
        match e {
            Some(value) => {
                println!("Vector {}: {}", i, value);
            }
            None => {
                println!("Vector {}: No existe", i);
            }
        }
    }
}
