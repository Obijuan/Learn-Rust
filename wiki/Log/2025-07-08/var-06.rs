//-- Documentacion de rust
//-- Imprimir valores máximos y mínimos de los tipos
//-- SALIDA EN HEXADECIMAL

fn main() {


    println!("───────────────────────────────────────────");
    println!(" TIPOS Y TAMAÑOS MÁXIMOS Y MÍNIMOS");
    println!("───────────────────────────────────────────");
    println!(" ENTEROS SIN SIGNO");
    println!("────────────────────────");
    println!("🟢  u8    ──> MAX: {:#X}", u8::MAX);
    println!("🟢  u16   ──> MAX: {:#X}", u16::MAX);
    println!("🟢  u32   ──> MAX: {:#X}", u32::MAX);
    println!("🟢  u64   ──> MAX: {:#X}", u64::MAX);
    println!("🟢  usize ──> MAX: {:#X}", usize::MAX);
    println!("🟢  u128  ──> MAX: {:#X}", u128::MAX);
    println!("────────────────────────");
    println!(" ENTEROS CON SIGNO");
    println!("────────────────────────");
    println!("🟢  i8: (MIN: {:#X}, MAX: {:#X})", i8::MIN, i8::MAX);
    println!("🟢  i16: (MIN: {:#X}, MAX: {:#X})", i16::MIN, i16::MAX);
    println!("🟢  i32: (MIN: {:#X}, MAX: {:#X})", i32::MIN, i32::MAX);
    println!("🟢  i64: (MIN: {:#X}, MAX: {:#X})", i64::MIN, i64::MAX);
    println!("🟢  isize: (MIN: {:#X}, MAX: {:#X})", isize::MIN, isize::MAX);
    println!("🟢  i128: (MIN: {:#X}, MAX: {:#X})", i128::MIN, i128::MAX);
    println!("");

}

