//-- Pruebas con arrays y bucles 

fn main() {

    //-- Array de 5 elementos
    let a = [10, 20, 30, 40, 50];

    //-- Imprimir el array completo
    println!("Array: {:?}", a);

    //-- Imprimir longitud del array
    println!("Longitud: {}", a.len());

    //-- Recorrer el array imprimiendo cada elemento
    for element in a.iter() {
        println!("Elemento: {}", element);
    }   

    //-- Recorrer el array usando el índice
    for i in 0..a.len() {
        println!("Elemento {} --> {}", i, a[i]); 
    }

}

