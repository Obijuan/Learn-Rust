//-- Pruebas de los tipos enumerados

//-- Definimos un enumerado simple
enum Test {
    Valor1,
    Valor2,
    Valor3,
}

fn main() {
    let a = Test::Valor1;

    //-- Los enumerados por defecto NO se pueden imprimir
    //-- Este codigo da un error de compilación 
    //-- Con :? se pueden imprimir variables para depuración
    println!("El valor de 'a' es: {:?}", a);
}
