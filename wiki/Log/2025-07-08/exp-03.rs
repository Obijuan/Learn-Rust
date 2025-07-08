//-- Más sobre literales y constantes en Rust

fn main() {

    //-- Definimos constantes
    const OCTAL: u8 = 0o10;
    const C1:u32 = 1_100;

    //-- En las literales se puede especificar el tipo
    //-- mediante un sufijo. ¡Es muy cómodo!
    let var1 = 0xFFu8;
    let var2 = 0xFFu16;

    //-- El underscore se puede usar siempre que queramos
    //-- con los numeros para mejorar la legibilidad
    let var3 = 0xAA_u8;
    const C2: u32 = 0xBEBE_CAFE;
    const C3: u32 = 0b0011_0000_0001_0010;
    const C4: u16 = 0xCA__FE;
    
    let var4 = 0xAA_usize;

    println!("🟢 Constante OCTAL: {}", OCTAL);
    println!("🟢 Constante C1: {}", C1);
    println!("🟢 Variable var1: {}", var1);
    println!("🟢 Variable var2: {}", var2);
    println!("🟢 Variable var3: {}", var3);
    println!("🟢 Constante C2: {}", C2);
    println!("🟢 Constante C3: {}", C3);
    println!("🟢 Variable var4: {}", var4);
    println!("🟢 Constante C4: {}", C4);
}
