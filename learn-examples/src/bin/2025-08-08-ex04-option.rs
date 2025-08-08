
fn main()
{
    //-- Valor opcional. Asignado valor 5
    let a: Option<u8> = Some(5);

    //-- Ningún valor asignado
    let b: Option<u8> = None;

    //-- Comprobar todos los casos de a
    match a {
        Some(valor) => {
            println!("🟢 Valor a: {}", valor); }
        None => {println!("🟢 Valor a: Ningún valor asignado");}
    }

    //-- Comprobar todos los casos de b
    match b {
        Some(valor) => {
            println!("🟢 Valor b: {}", valor); }
        None => {println!("🟢 Valor b: Ningún valor asignado");}
    }

}

