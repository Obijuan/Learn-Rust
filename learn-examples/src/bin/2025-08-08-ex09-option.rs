
//-- Recibir un valor (opcional) e imprimir su valor
//-- Se devuelve el propio valor recibido
//-- Es una especie de funcion identidad
fn id(a: Option<u8>) -> Option<u8> 
{
    //-- Obtener el valor de a, si existe
    //-- De lo contrario se retorna None
    let c = match a {
        Some(valor) => { valor }
        None => return None
    };

    println!("Valor: {}", c);

    return Some(c);
}

fn main()
{
    let a: Option<u8> = Some(5);
    let b: Option<u8> = None;

    //-- Imprimir el valor de a, si existe
    id(a);

    //-- Imprimir el valor de b, si existe
    id(b);

}

