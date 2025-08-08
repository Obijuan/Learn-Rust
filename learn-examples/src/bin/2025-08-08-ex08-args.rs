
fn main()
{
    //-- Obtener el iterador de los argumentos
    let args = std::env::args();

    println!("➡️ Argumentos:");
    println!("  🟢 Cantidad: {}", args.len());

    for (i, arg) in args.enumerate() {
        println!("  🟢 Argumento {}: {}",i,arg);
    }

}

