//-- Pruebas de los tipos enumerados

//-- Definimos un enumerado simple

//-- Con este atributo el compilador añade los métodos necesarios al 
//-- tipo Test para que se pueda imprimir
//-- Este atributo sólo afecta al enum Test
#[derive(Debug)]
enum Test {
    Valor1,
    Valor2,
    Valor3,
}

fn main() {
    let a = Test::Valor1;
    let b = Test::Valor2;
    let c = Test::Valor3;

    //-- Imprimir el tipo enumerado
    //-- Ahora ya NO da error
    println!("El valor de 'a' es: {:?}", a);
    println!("El valor de 'b' es: {:?}", b);
    println!("El valor de 'c' es: {:?}", c);
}
