//-- Ejemplo de conversiones de tipos

fn main() {

    //-- varx: tipos originales
    //-- vary: tipos convertidos

    let varx1: i32 = 42;
    let vary1: u32 = varx1 as u32;

    let varx2: i8 = -1;
    let vary2: u8 = varx2 as u8;

    let vary3: u16 = varx2 as u16;

    let vary4: u32 = varx2 as u32;

    //-- Ejemplos de truncado
    let varx3: u32 = 0xCAFE_BACA;
    let vary5: u16 = varx3 as u16;
    let vary6: u8 = varx3 as u8;

    //-- Imprimimos los resultados
    println!("🟢 varx1,vary1: {}, {}", varx1, vary1);
    println!("🟢 varx2,vary2: {}, {}", varx2, vary2);
    println!("🟢 varx2,vary3: {}, {}", varx2, vary3);
    println!("🟢 varx2,vary4: {}, {}", varx2, vary4);
    println!("🟢 varx3,vary5: {:#x}, {:#x}", varx3, vary5);
    println!("🟢 varx3,vary6: {:#x}, {:#x}", varx3, vary6);

}
