//-- Pruebas de los tipos enumerados

//-- Definimos un enumerado simple
enum Test {
    Valor1,
    Valor2,
    Valor3,
}

fn main() {

    //-- Los tipos enumerados tiene valores por defecto (son numeros)
    //-- Podmeos usar su valor cambiando al tipo i32
    println!("Valor1: {}", Test::Valor1 as i32);
    println!("Valor2: {}", Test::Valor2 as i32);
    println!("Valor3: {}", Test::Valor3 as i32);
}

