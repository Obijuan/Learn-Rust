//-- Pruebas de los tipos enumerados

//-- Enumerado para los opcodes de un RISC-V
enum Opcode {
    TipoR = 0b_01100_11,
    TipoS = 0b_01000_11,
    TipoB = 0b_11000_11,
    TipoUlui = 0b_01101_11,
    TipoUauipc = 0b_00101_11,
    TipoJjal = 0b_11011_11,
    TipoJjalr = 0b_11001_11,
    TipoEcallEbreak = 0b_11100_11,
}

fn main() {

    //-- Print Opcodes
    println!("");
    println!("Opcodes RISC-V:");
    println!("──────────────");
    println!("🟢 Tipo-R: {:#04X}", Opcode::TipoR as u8);
    println!("🟢 Tipo-S: {:#04X}", Opcode::TipoS as u8);
    println!("🟢 Tipo-B: {:#04X}", Opcode::TipoB as u8);
    println!("🟢 Tipo-U-lui: {:#04X}", Opcode::TipoUlui as u8);
    println!("🟢 Tipo-U-auipc: {:#04X}", Opcode::TipoUauipc as u8);
    println!("🟢 Tipo-J-jal: {:#04X}", Opcode::TipoJjal as u8);
    println!("🟢 Tipo-J-jalr: {:#04X}", Opcode::TipoJjalr as u8);
    println!("🟢 TipoEcallEbreak: {:#04X}", Opcode::TipoEcallEbreak as u8);
    println!("");
   
}

