//-- Pruebas con arrays
//-- Los arrays en rust tienen siempre tamaño fijo
//-- y su tipo es homogéneo

fn main() {

    //-- Array de 5 elementos
    let a = [10, 20, 30, 40, 50];

    //-- Imprimir el array completo
    //-- NOTA: hay que usar {:?} para imprimir arrays
    println!("Array: {:?}", a);

    //-- Acceder a los elementos individuales, mediante
    //-- su índice que empieza por 0
    println!("Elemento 0: {}", a[0]);
    println!("Elemento 1: {}", a[1]);
    println!("Elemento 2: {}", a[2]);
    println!("Elemento 3: {}", a[3]);
    println!("Elemento 4: {}", a[4]);
}

