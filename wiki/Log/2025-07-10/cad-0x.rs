//-- Ejemplos de cadenas

fn main() {

    let cad1 = "🟢Hola!!🙂";

    //-- Imprimir la cadena
    println!("Valor de cad1: {}", cad1);

    //-- Imprimir su tamaño
    println!("Tamaño de cad1: {} bytes", std::mem::size_of_val(&cad1));

    //-- Tamaño de la cadena en caracteres
    let tam_cad1 = cad1.chars().count();
    println!("Tamaño de cad1 en caracteres: {}", tam_cad1);

}


/* 

    //-- Ejemplo de cadena
    let cadena1: &str = "Hola, Rust!";
    
    //-- Tamaño de cadena1
    println!("Tamaño de cadena1: {} bytes", std::mem::size_of_val(&cadena1));
    
    //-- Imprimir la cadena
    println!("Valor de cadena1: {}", cadena1);
    
    //-- Imprimir cada carácter de la cadena
    for c in cadena1.chars() {
        println!("Carácter: {}", c);
    }
*/   