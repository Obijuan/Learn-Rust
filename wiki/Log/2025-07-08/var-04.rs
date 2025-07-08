//-- Documentacion de rust
//-- Imprimir los tipos y sus tamaños 

fn main() {


    println!("───────────────────────────────────────────");
    println!(" TIPOS Y TAMAÑOS DE VARIABLES en RUST");
    println!("───────────────────────────────────────────");
    println!(" ENTEROS SIN SIGNO");
    println!("────────────────────────");
    println!("🟢  u8    ──> {} byte", std::mem::size_of::<u8>());
    println!("🟢  u16   ──> {} bytes", std::mem::size_of::<u16>());
    println!("🟢  u32   ──> {} bytes", std::mem::size_of::<u32>());
    println!("🟢  u64   ──> {} bytes", std::mem::size_of::<u64>());
    println!("🟢  usize ──> {} bytes", std::mem::size_of::<usize>());
    println!("🟢  u128  ──> {} bytes", std::mem::size_of::<u128>());
    println!("────────────────────────");
    println!(" ENTEROS CON SIGNO");
    println!("────────────────────────");
    println!("🟢  i8    ──> {} byte", std::mem::size_of::<i8>());
    println!("🟢  i16   ──> {} bytes", std::mem::size_of::<i16>());
    println!("🟢  i32   ──> {} bytes", std::mem::size_of::<i32>());
    println!("🟢  i64   ──> {} bytes", std::mem::size_of::<i64>());
    println!("🟢  isize ──> {} bytes", std::mem::size_of::<isize>());
    println!("🟢  i128  ──> {} bytes", std::mem::size_of::<i128>());


    println!("");

}

