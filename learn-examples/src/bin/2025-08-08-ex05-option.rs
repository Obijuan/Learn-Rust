
fn main()
{
    //-- Valor opcional. Asignado valor 5
    let a: Option<u8> = Some(5);

    //-- Ningún valor asignado
    let b: Option<u8> = None;

    //-- Comprobar todos los casos de a
    if let Some(valor) = a {
        println!("🟢 Valor a: {}", valor);
    }
    else {
        println!("🟢 Valor a: Ningún valor asignado");
    }

    //-- Comprobar todos los casos de b
    if let Some(valor) = b {
        println!("🟢 Valor b: {}", valor);
    }
    else {
        println!("🟢 Valor b: Ningún valor asignado");
    }

}

