//-- Trabajando con expresiones
//-- Mini-desensamblador RISC-V

fn main() {

    //-- Consatantes: Instrucciones RISC-V
    const INST1: u32 = 0x00000013;  //-- NOP: addi x0, x0, 0
    const INST2: u32 = 0x0aa00093;  //-- add x1, x0, 0xAA

    //-- Posiciones de los campos en la instruccion
    const OPCODE_POS: u8 = 0;       //-- Posición del opcode
    const RD_POS: u8 = 7;
    const FUNC3_POS: u8 = 12;  //-- Posición del campo func3
    const RS1_POS: u8 = 15;  //-- Posición del registro fuente 1
    const RS2_POS: u8 = 20;  //-- Posición del registro fuente 2
    const FUNC7_POS: u8 = 25;  //-- Posición del campo func7
    const IMM12_POS: u8 = 20;  //-- Posición del campo inmediato de 12 bits
    const FIELD_3B: u32 = 0x07;  //-- Campo de 3 bits
    const FIELD_5B: u32 = 0x1F;  //-- Campo de 5 bits
    const FIELD_7B: u32 = 0x7F;  //-- Campo de 7 bits
    const FIELD_12B: u32 = 0xFFF;  //-- Campo de 12 bits

    //-- Máscaras para extraer campos de la instrucción
    const OPCODE_MASK: u32 = FIELD_7B << OPCODE_POS;  //-- Máscara del opcode
    const RD_MASK: u32 = FIELD_5B << RD_POS;  //-- Registro destino
    const FUNC3_MASK: u32 = FIELD_3B << FUNC3_POS;  //-- Máscara del campo func3
    const RS1_MASK: u32 = FIELD_5B << RS1_POS;  //-- Registro fuente 1
    const RS2_MASK: u32 = FIELD_5B << RS2_POS;
    const FUNC7_MASK: u32 = FIELD_7B << FUNC7_POS;
    const IMM12_MASK: u32 = FIELD_12B << IMM12_POS;  //-- Máscara del campo inmediato de 12 bits

    //-- {:#010X}
    //--   # --> Imprimir prefijo 0x
    //--   0 --> Rellenar con ceros por la izquierda
    //--   10 --> Tamaño total del numero: 10 caracteres (8 digitos + 2 de 0x)
    //--   X --> Imprimir en hexadecimal (mayúsculas)
    println!("🟢 Instrucción: {:#010X}", INST1);

    let opcode_1 = (INST1 & OPCODE_MASK) >> OPCODE_POS;
    let rd_1 = (INST1 & RD_MASK) >> RD_POS;
    let func3_1 = (INST1 & FUNC3_MASK) >> FUNC3_POS;
    let rs1_1 = (INST1 & RS1_MASK) >> RS1_POS;
    let _rs2_1 = (INST1 & RS2_MASK) >> RS2_POS;
    let imm_1 = (INST1 & IMM12_MASK) >> IMM12_POS;
    let _func7_1 = (INST1 & FUNC7_MASK) >> FUNC7_POS;
    
    println!("   - Opcode: {:#4X}", opcode_1);
    println!("   - rd: x{}", rd_1);
    println!("   - func3: {:#05b}", func3_1);
    println!("   - rs1: x{}", rs1_1);
    println!("   - Inmediato: {:#X}", imm_1);


    println!("🟢 Instrucción: {:#010X}", INST2);
    let opcode_2 = (INST2 & OPCODE_MASK) >> OPCODE_POS;
    let rd_2 = (INST2 & RD_MASK) >> RD_POS;
    let func3_2 = (INST2 & FUNC3_MASK) >> FUNC3_POS;
    let rs1_2 = (INST2 & RS1_MASK) >> RS1_POS;
    let _rs2_2 = (INST2 & RS2_MASK) >> RS2_POS;
    let imm_2 = (INST2 & IMM12_MASK) >> IMM12_POS;
    let _func7_2 = (INST2 & FUNC7_MASK) >> FUNC7_POS;
    println!("   - Opcode: {:#4X}", opcode_2);
    println!("   - rd: x{}", rd_2);
    println!("   - func3: {:#05b}", func3_2);
    println!("   - rs1: x{}", rs1_2);
    println!("   - Inmediato: {:#X}", imm_2);
}

