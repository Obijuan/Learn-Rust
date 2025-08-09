fn main() 
{
    let v: Vec<&str> = vec!["cad1", "cad2", "cad3"];

    //-- Imprimir longitud del vector
    println!("Numero de bytes: {}", v.len());

    //-- Imprimir los bytes
    print!("Vector: ");
    for cad in v {
        print!("{} ", cad);
    }
    println!("");
}
