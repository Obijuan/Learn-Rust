//────────────────────────────────────────────────//
// Programa ejecutable
// Esto es lo que se compila y ejecuta en modo 
// normal
//────────────────────────────────────────────────//


//-- Definimos la función de incremento
fn inc(x: u8) -> u8 {
    //-- Incrementamos el valor de x
    x + 1
}


fn main() {

    println!("Programa Main:");
    println!("Prueba de la función inc");
    println!("Valor: {}, inc: {}", 5, inc(5));

}

//────────────────────────────────────────────────//
//  Pruebas unitarias
//  Estas funciones se ejecuta en modo test
//  En este modo el programa no se ejecuta
//────────────────────────────────────────────────//

#[test]
fn test_inc() {
//────────────────────────────────────────────────//
//  Prueba unitaria para la funcion inc
//────────────────────────────────────────────────//
    assert_eq!(inc(5), 6);
    assert_eq!(inc(0), 1);
    assert_eq!(inc(127), 128);
    assert_eq!(inc(100), 101);
}

