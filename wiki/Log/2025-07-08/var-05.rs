//-- Documentacion de rust
//-- Imprimir valores máximos y mínimos de los tipos

fn main() {


    println!("───────────────────────────────────────────");
    println!(" TIPOS Y TAMAÑOS MÁXIMOS Y MÍNIMOS");
    println!("───────────────────────────────────────────");
    println!(" ENTEROS SIN SIGNO");
    println!("────────────────────────");
    println!("🟢  u8    ──> MAX: {}", u8::MAX);
    println!("🟢  u16   ──> MAX: {}", u16::MAX);
    println!("🟢  u32   ──> MAX: {}", u32::MAX);
    println!("🟢  u64   ──> MAX: {}", u64::MAX);
    println!("🟢  usize ──> MAX: {}", usize::MAX);
    println!("🟢  u128  ──> MAX: {}", u128::MAX);
    println!("────────────────────────");
    println!(" ENTEROS CON SIGNO");
    println!("────────────────────────");
    println!("🟢  i8: (MIN: {}, MAX: {})", i8::MIN, i8::MAX);
    println!("🟢  i16: (MIN: {}, MAX: {})", i16::MIN, i16::MAX);
    println!("🟢  i32: (MIN: {}, MAX: {})", i32::MIN, i32::MAX);
    println!("🟢  i64: (MIN: {}, MAX: {})", i64::MIN, i64::MAX);
    println!("🟢  isize: (MIN: {}, MAX: {})", isize::MIN, isize::MAX);
    println!("🟢  i128: (MIN: {}, MAX: {})", i128::MIN, i128::MAX);
    println!("");

}

