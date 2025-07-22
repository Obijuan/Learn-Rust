//-- Pruebas de los tipos enumerados

//-- Definimos un enumerado simple
enum Test {
    Valor1,
    Valor2,
    Valor3,
}

fn main() {
    let a:Test = Test::Valor1;

    //-- Imprimir el tipo enumerado
    //-- Con match podemos hacer una cosa u otra en función
    //-- del valor de la variable
    match a {
        Test::Valor1 => println!("El valor es Valor1"),
        Test::Valor2 => println!("El valor es Valor2"),
        Test::Valor3 => println!("El valor es Valor3"),
    }
}

