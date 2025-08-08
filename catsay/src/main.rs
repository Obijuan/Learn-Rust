fn main() {

    //-- Lectura del primer argumento
    let msg = std::env::args().nth(1)
        .expect("Mensaje NO especificado. Uso: catsay < mensaje >");
    println!();
    println!("🟢 Mensaje:");
    println!("──────────────────────────────");
    println!("{}", msg);
    println!("──────────────────────────────");
    println!();
}
