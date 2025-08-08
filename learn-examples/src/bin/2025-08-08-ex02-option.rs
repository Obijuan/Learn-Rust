
fn main()
{
    //-- Valor opcional. Asignado valor 5
    let a: Option<u8> = Some(5);

    //-- Valor opcional. Ninguno asignado
    let b: Option<u8> = None;

    //-- Con unwrap_or se optiene el valor
    println!("🟢 Valor a: {}", a.unwrap());
    println!("🟢 Valor b: {}", b.unwrap());
}
