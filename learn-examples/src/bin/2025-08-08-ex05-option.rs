
fn main()
{
    //-- Valor opcional. Asignado valor 5
    let a: Option<u8> = Some(5);

    //-- Ningún valor asignado
    let b: Option<u8> = None;

    //-- Comprobar todos los casos de a
    println!("🟢 Valor a: {}", a.expect("No asignado"));
    

    //-- Comprobar todos los casos de b
    println!("🟢 Valor b: {}", b.expect("No asignado"));

    println!("Este mensaje NO se imprime porque se aborta antes")
}

