//-- Impresión en diferentes bases

fn main() {

    //-- Consatante sin tipo definido
    const A: u8 = 20;
    const B: u16 = 30;
    const C: u32 = 40;

    println!("🟢 Constante A: {}, tamaño: {}", A, std::mem::size_of_val(&A));
    println!("🟢 Constante B: {}, tamaño: {}", B, std::mem::size_of_val(&B));
    println!("🟢 Constante C: {}, tamaño: {}", C, std::mem::size_of_val(&C));
}

