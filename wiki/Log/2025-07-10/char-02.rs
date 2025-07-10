//-- Pruebas con el tipo char

fn main() {

    //-- Variable de tipo char
    let a: char = 'a';

    println!("El valor de a es: {}", a);

    //-- Imprimir el tamaño de la variable
    println!("El tamaño de a es: {} bytes", std::mem::size_of_val(&a));
    println!("El tamaño de char es: {} bytes", std::mem::size_of::<char>());    
}


