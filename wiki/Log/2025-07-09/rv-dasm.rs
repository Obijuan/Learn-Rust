//────────────────────────────────────────────────
//  rv-dasm.rs
//  Desensamblador de instrucciones RISC-V
//  Autor: Obijuan
//  Fecha: 09/07/2025
//  Licencia: CC BY-SA 4.0
//────────────────────────────────────────────────


//────────────────────────────────────────────────
//  CONSTANTES PARA ACCESO A LA ISA DEL RISCV   
//────────────────────────────────────────────────
//  Definir anchura de campos. Estos campos son los que luego se llevan
//  a la posición concreta en la instrucción, para calcular la máscara
//  con la que extraer el campo
//───────────────────────────
//  ANCHURAS de LOS CAMPOS
//───────────────────────────
const FIELD_3B: u32 = 0x07;  //-- Campo de 3 bits de ancho
const FIELD_5B: u32 = 0x1F;  //-- Campo de 5 bits
const FIELD_7B: u32 = 0x7F;  //-- Campo de 7 bits
const FIELD_12B: u32 = 0xFFF;  //-- Campo de 12 bits
//────────────────────────────────────────────────
//  POSICIONES de LOS CAMPOS
//────────────────────────────────────────────────
const OPCODE_POS: u8 = 0;  
const RD_POS: u8 = 7;
const FUNC3_POS: u8 = 12;  
const RS1_POS: u8 = 15;  
const RS2_POS: u8 = 20;  
const FUNC7_POS: u8 = 25;  
const IMM12_POS: u8 = 20;  
//────────────────────────────────────────────────
//  CALCULAR LAS MASCARAS DE ACCESO A LOS CAMPOS
//────────────────────────────────────────────────
//  Se calculan desplazando los campos de la anchura correspondiente
//  a la posición del campo
//──────────────────────────────────────────────
const OPCODE_MASK: u32 = FIELD_7B << OPCODE_POS; 
const RD_MASK: u32 = FIELD_5B << RD_POS;  
const FUNC3_MASK: u32 = FIELD_3B << FUNC3_POS;  
const RS1_MASK: u32 = FIELD_5B << RS1_POS;  
const RS2_MASK: u32 = FIELD_5B << RS2_POS;
const FUNC7_MASK: u32 = FIELD_7B << FUNC7_POS;
const IMM12_MASK: u32 = FIELD_12B << IMM12_POS;  

fn get_opcode(inst: u32) -> u32 {
//────────────────────────────────────────────────
// Entrada: Instrucción RISC-V
// Salida: Opcode de la instrucción
//────────────────────────────────────────────────
  //-- Aplicar la máscara para extraer el campo
  //-- y desplazarlo a la posición 0
  (inst & OPCODE_MASK) >> OPCODE_POS
}

fn get_rd(inst: u32) -> u32 {
//────────────────────────────────────────────────
// Entrada: Instrucción RISC-V
// Salida: Registro destino (rd) de la instrucción
//────────────────────────────────────────────────
  //-- Aplicar la máscara para extraer el campo
  //-- y desplazarlo a la posición 0
  (inst & RD_MASK) >> RD_POS
}

fn get_func3(inst: u32) -> u32 {
//────────────────────────────────────────────────
// Entrada: Instrucción RISC-V
// Salida: Func3 de la instrucción
//────────────────────────────────────────────────
  //-- Aplicar la máscara para extraer el campo
  //-- y desplazarlo a la posición 0
  (inst & FUNC3_MASK) >> FUNC3_POS
}

fn get_rs1(inst: u32) -> u32 {
//────────────────────────────────────────────────
// Entrada: Instrucción RISC-V          
// Salida: Registro fuente 1 (rs1) de la instrucción
//────────────────────────────────────────────────
  //-- Aplicar la máscara para extraer el campo
  //-- y desplazarlo a la posición 0
  (inst & RS1_MASK) >> RS1_POS
}

fn get_rs2(inst: u32) -> u32 {
//────────────────────────────────────────────────
// Entrada: Instrucción RISC-V
// Salida: Registro fuente 2 (rs2) de la instrucción
//────────────────────────────────────────────────
  //-- Aplicar la máscara para extraer el campo
  //-- y desplazarlo a la posición 0    
    (inst & RS2_MASK) >> RS2_POS    
}

fn get_imm12(inst: u32) -> u32 {
//────────────────────────────────────────────────
// Entrada: Instrucción RISC-V
// Salida: Inmediato de 12 bits de la instrucción
//────────────────────────────────────────────────
  //-- Aplicar la máscara para extraer el campo
  //-- y desplazarlo a la posición 0
  (inst & IMM12_MASK) >> IMM12_POS
}

fn get_func7(inst: u32) -> u32 {
//────────────────────────────────────────────────
// Entrada: Instrucción RISC-V
// Salida: Func7 de la instrucción
//────────────────────────────────────────────────
  //-- Aplicar la máscara para extraer el campo
  //-- y desplazarlo a la posición 0
  (inst & FUNC7_MASK) >> FUNC7_POS
}

fn print_fields(inst: u32) {
//────────────────────────────────────────────────
// Entrada: Instrucción RISC-V
// Salida: Imprime los campos de la instrucción
//────────────────────────────────────────────────

    //-- Extraer los campos de la instrucción
    let opcode = get_opcode(inst);
    let rd = get_rd(inst);
    let func3 = get_func3(inst);
    let rs1 = get_rs1(inst);
    let rs2 = get_rs2(inst);
    let imm = get_imm12(inst);
    let func7 = get_func7(inst);

    //-- Imprimir los campos extraídos
    println!("   - Opcode: {:#4X}", opcode);
    println!("   - rd: x{}", rd);
    println!("   - func3: {:#05b}", func3);
    println!("   - rs1: x{}", rs1);
    println!("   - rs2: x{}", rs2);
    println!("   - Inmediato: {:#X}", imm);
    println!("   - Func7: {:#07b}", func7);
}
//────────────────────────────────────────────────


//────────────────────────────────────────────────
//  PROGRAMA PRINCIPAL
//────────────────────────────────────────────────
fn main() {

    //-- Consatantes: Instrucciones RISC-V
    const INST1: u32 = 0x00000013;  //-- NOP: addi x0, x0, 0
    const INST2: u32 = 0x0aa00093;  //-- add x1, x0, 0xAA    

    //-- Mostrar la instrucción actual
    println!("🟢 Instrucción: {:#010X}", INST1);

    //-- Imprimir los campos
    print_fields(INST1);

    println!("🟢 Instrucción: {:#010X}", INST2);
    print_fields(INST2);
}

//────────────────────────────────────────────────
//  TESTS
//────────────────────────────────────────────────

#[test]
fn test_get_opcode() {
    //-- Test de la función get_opcode

    //-- Instrucciones reales
    assert_eq!(get_opcode(0x0000_0013), 0x13);
    assert_ne!(get_opcode(0x0000_0013), 0x00);
    assert_eq!(get_opcode(0x0aa0_0093), 0x13);

    //-- Instrucciones inventadas
    assert_eq!(get_opcode(0xffff_ffff), 0x7f);
    assert_eq!(get_opcode(0x0000_0000), 0x00);
    assert_eq!(get_opcode(0xaaaa_aaaa), 0x2a);
    assert_eq!(get_opcode(0x0000_0000_0000_0000__0000_00000_0000001), 0x01);
    assert_eq!(get_opcode(0x0000_0000 | 0b0000001), 0x01);
}

#[test]
fn test_get_rd() {
    //-- Test de la función get_rd

    //-- Instrucciones reales
    assert_eq!(get_rd(0x0000_0013), 0);
    assert_eq!(get_rd(0x0aa0_0093), 1);

    //-- Instrucciones inventandas
    assert_eq!(get_rd(0x0000_0000 | 0b00000_0000000), 0);
    assert_eq!(get_rd(0x0000_0000 | 0b00001_0000000), 1);
    assert_eq!(get_rd(0xFFFF_0000 | 0b00010_0000000), 2); 
    assert_eq!(get_rd(0xFFFF_0000 | 0b00100_0000000), 4); 
    assert_eq!(get_rd(0xFFFF_0000 | 0b01000_0000000), 8); 
    assert_eq!(get_rd(0xFFFF_0000 | 0b10000_0000000), 16);
    assert_eq!(get_rd(0xFFFF_0000 | 0b10001_0000000), 17); 
    assert_eq!(get_rd(0xFFFF_0000 | 0b11111_0000000), 31); 
}

#[test]
fn test_get_func3() {
  //-- Test de la función get_func3

  assert_eq!(get_func3(0b_0000000_00000_00000_000_00000_0000000), 0b000);
  assert_eq!(get_func3(0b_0000000_00000_00000_001_00000_0000000), 0b001);
  assert_eq!(get_func3(0b_0000000_00000_00000_010_00000_0000000), 0b010);
  assert_eq!(get_func3(0b_0000000_00000_00000_100_00000_0000000), 0b100);
  assert_eq!(get_func3(0b_0000000_00000_00000_111_00000_0000000), 0b111);
}

