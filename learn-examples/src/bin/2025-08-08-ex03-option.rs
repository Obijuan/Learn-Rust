
fn main()
{
    //-- Valor opcional. Asignado valor 5
    let a: Option<u8> = Some(5);

    //-- Valor opcional. Ninguno asignado
    let b: Option<u8> = None;

    //-- Valor por defecto
    const DEFAULT:u8 = 0;

    //-- Con unwrap_or se optiene el valor. si es None
    //-- se devuelve un valor por defecto
    println!("🟢 Valor a: {}", a.unwrap_or(DEFAULT));
    println!("🟢 Valor b: {}", b.unwrap_or(DEFAULT));
}
