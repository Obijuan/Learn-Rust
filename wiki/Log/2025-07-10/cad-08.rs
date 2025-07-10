
fn int_to_str(value: i32) -> String {
//------------------------------------------
//-- Convertir un numero a una cadena
//-- ENTRADA: valor
//-- SALIDA: Cadena
//-----------------------------------------
    format!("{}", value)
}

fn main() {

    //-- Convertir un entero
    let cad = int_to_str(255);

    //-- Imprimirlo
    println!("{}", cad)
}
