//────────────────────────────────────────────────
//  rv-dasm.rs
//  Desensamblador de instrucciones RISC-V
//  Autor: Obijuan
//  Fecha: 09/07/2025
//  Licencia: CC BY-SA 4.0
//────────────────────────────────────────────────

//-- Instrucciones RV32I
//-- https://msyksphinz-self.github.io/riscv-isadoc/html/rvi.html

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
//────────────────────────────────────────────────
//  DEFINICION DE LOS OPCODES
//────────────────────────────────────────────────
//  Instrucciones tipo-I
//────────────────────────────
//-- Estas instrucciones se dividen a su vez en dos grupos:
//  - Instrucciones aritméticas (ADDI, ANDI, ORI,...)
//  - Instrucciones de carga (LW, LH, LB,...)
const OPCODE_I_ARITH: u32 = 0x13;   //-- ADDI: addi rd, rs1, imm12
const OPCODE_I_LOAD: u32 = 0x03;    //-- LW: lw rd, imm12(rs1)

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

fn get_imm12(inst: u32) -> i32 {
//────────────────────────────────────────────────
// Entrada: Instrucción RISC-V
// Salida: Inmediato de 12 bits de la instrucción
//────────────────────────────────────────────────
  //-- Aplicar la máscara para extraer el campo
  //-- y desplazarlo a la posición 0
  let imm12: u32 = (inst & IMM12_MASK) >> IMM12_POS;

  //-- Convertir el valor a i32 para manejar el signo
  //-- y devolverlo!
  sign_ext(imm12 as i32)
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

fn sign_ext(value: i32) -> i32 {
//────────────────────────────────────────────────
// Entrada: Valor de 12 bits  
// Salida: Valor extendido a 32 bits con signo
//────────────────────────────────────────────────
    //-- Obtener el bit de signo
    //-- sign_bit = true --> negativo
    let sign_bit = (value & 0x800) != 0;

    //-- En caso de ser negativo, extender el signo
    if sign_bit {
        value | !0xFFF  //-- Extender el signo a 32 bits
    } else {
        value  //-- No es negativo, devolver el valor original
    }
}

fn is_type_i(opcode: u32) -> bool {
//────────────────────────────────────────────────
// Entrada: opcode RISC-V
// Salida: true si es una instrucción tipo I
//────────────────────────────────────────────────
    //-- Las instrucciones de tipo I son bien las de tipo
    //-- aritmético (ADDI, ANDI, ORI,...) o las de LOAD (LW, LH, LB,...)
    if opcode == OPCODE_I_ARITH || opcode == OPCODE_I_LOAD {
        true
    } else {
        false
    }
}

fn is_type_i_arith(opcode: u32) -> bool {
    if opcode == OPCODE_I_ARITH {
        true
    }
    else {
        false
    }
}

fn is_type_i_load(opcode: u32) -> bool {
  if opcode == OPCODE_I_LOAD {
    true
  }
  else {
    false
  }
}

fn inst_type_i_arith(func3: u32) -> String {
//────────────────────────────────────────────────
//  Obtener el nombre de la instruccion aritmetica
//  de tipo i cuyo codigo func3 es el dado
//  ENTRADA: Codigo func3
//  SALIDA: nemonico
//────────────────────────────────────────────────

    //-- Tabla de las instrucciones aritméticas
    //-- y lógicas con valores inmediatos
    let name = 
    [          //-- Func3
      "addi",  //-- 000
      "slli",  //-- 001
      "slti",  //-- 010
      "sltiu", //-- 011
      "xori",  //-- 100
      "srli",  //-- 101
      "ori",   //-- 110
      "andi",  //-- 111
    ];

    //-- Devolver la cadena a partir del código
    name[func3 as usize].to_string()
}


fn disassemble(inst: u32) -> String {
//────────────────────────────────────────────────
// Desensamblar una instruccion en codigo maquina
// Entrada: Instrucción en codigo maquina RISC-V
// Salida: Cadena con la instrucción en ensamblador
//────────────────────────────────────────────────

    //-- Extraer el opcode de la instrucción  
    let opcode = get_opcode(inst);

    //-- Comprobar el tipo de instrucción que es, según el opcode
    if is_type_i(opcode) {

        //-- Sabemos el formato, podemos obtener todos los campos
        let func3 = get_func3(inst);

        //-- Distinguir entre el tipo I de instruccion:
        //--   - Instrucciones aritmeticas (ADDI, ANDI, ORI,...)
        //--   - Instrucciones de carga (LW, LH, LB,...)
        if is_type_i_arith(opcode) {

            let rd = get_rd(inst);
            let rs1 = get_rs1(inst);
            let imm = get_imm12(inst) as i32;

            //-- Nombre de la instruccion            
            let name = inst_type_i_arith(func3);
            format!("{} x{}, x{}, {}", name, rd, rs1, imm)

        } else if is_type_i_load(opcode) {
            let rd = get_rd(inst);
            let rs1 = get_rs1(inst);
            let imm = get_imm12(inst);
            format!("     * I-LOAD: x{} = Mem[x{} + {:#X}]", rd, rs1, imm)
        } else {
            println!("   - Instrucción: DESCONOCIDA");
            print_fields(inst);
            String::from("DESCONOCIDA")
        }

    } else {
        println!("   - Instrucción: DESCONOCIDA");
        print_fields(inst);
        String::from("DESCONOCIDA")
    }
}


//────────────────────────────────────────────────
//  PROGRAMA PRINCIPAL
//────────────────────────────────────────────────
fn main() {

    //-- Instrucciones RISC-V a desensamblar
    let insts = [
        0x00100093, // addi x1, x0, 1
        0x00111093, // slli x1, x2, 1
        0x00112093, // slti x1, x2, 1
        0x00113093, // sltiu x1, x2, 1

    ];

    for i in 0..insts.len() {

        let machine_code: u32 = insts[i];

        //-- Pasar la instruccion a String
        let inst: String = disassemble(insts[i]);

        //-- Imprimirla!
        println!("🟢 [{machine_code:#010X}]: {inst}");

    }

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

#[test]
fn test_get_rs1() {
  //-- Test de la función get_rs1 

  //--                   func7  rs2   rs1  func3 rd    opcode
  assert_eq!(get_rs1(0b_0000000_00000_00000_000_00000_0000000), 0);
  assert_eq!(get_rs1(0b_0000000_00000_00001_000_00000_0000000), 1);
  assert_eq!(get_rs1(0b_0000000_00000_00010_000_00000_0000000), 2);
  assert_eq!(get_rs1(0b_0000000_00000_00100_000_00000_0000000), 4);
  assert_eq!(get_rs1(0b_0000000_00000_01000_000_00000_0000000), 8);
  assert_eq!(get_rs1(0b_0000000_00000_10000_000_00000_0000000), 16);
  assert_eq!(get_rs1(0b_0000000_00000_10001_000_00000_0000000), 17);
  assert_eq!(get_rs1(0b_0000000_00000_11111_000_00000_0000000), 31);
}

#[test]
fn test_get_rs2() {
  //-- Test de la función get_rs2
  //--                   func7  rs2   rs1  func3 rd    opcode
  assert_eq!(get_rs2(0b_0000000_00000_00000_000_00000_0000000), 0);
  assert_eq!(get_rs2(0b_0000000_00001_00000_000_00000_0000000), 1);
  assert_eq!(get_rs2(0b_0000000_00010_00000_000_00000_0000000), 2);
  assert_eq!(get_rs2(0b_0000000_00100_00000_000_00000_0000000), 4);
  assert_eq!(get_rs2(0b_0000000_01000_00000_000_00000_0000000), 8);
  assert_eq!(get_rs2(0b_0000000_10000_00000_000_00000_0000000), 16);
  assert_eq!(get_rs2(0b_0000000_10001_00000_000_00000_0000000), 17);
  assert_eq!(get_rs2(0b_0000000_11111_00000_000_00000_0000000), 31);
}

#[test]
fn test_get_func7() {
  //-- Test de la función get_func7
  //--                   func7  rs2   rs1  func3 rd    opcode
  assert_eq!(get_func7(0b_0000000_00000_00000_000_00000_0000000), 0b0000000);
  assert_eq!(get_func7(0b_0000001_00000_00000_000_00000_0000000), 0b0000001);
  assert_eq!(get_func7(0b_0000010_00000_00000_000_00000_0000000), 0b0000010);
  assert_eq!(get_func7(0b_0000100_00000_00000_000_00000_0000000), 0b0000100);
  assert_eq!(get_func7(0b_0001000_00000_00000_000_00000_0000000), 0b0001000);
  assert_eq!(get_func7(0b_0010000_00000_00000_000_00000_0000000), 0b0010000);
  assert_eq!(get_func7(0b_0100000_00000_00000_000_00000_0000000), 0b0100000);
  assert_eq!(get_func7(0b_1000000_00000_00000_000_00000_0000000), 0b1000000);
  assert_eq!(get_func7(0b_1111111_00000_00000_000_00000_0000000), 0b1111111);
}

#[test]
fn test_get_imm12() {
  //-- Test de la función get_imm12
  //--                    ----imm12-----   rs1  func3  rd    opcode
  assert_eq!(get_imm12(0b_0000_0000_0000__00000__000__00000__0000000), 0b0000_0000_0000);
  assert_eq!(get_imm12(0b_0000_0000_0001__00000__000__00000__0000000), 0x001);
  assert_eq!(get_imm12(0x001_0_0000), 0x001); 
  assert_eq!(get_imm12(0x002_0_0000), 0x002);
  assert_eq!(get_imm12(0x004_0_0000), 0x004);
  assert_eq!(get_imm12(0x008_0_0000), 0x008);
  assert_eq!(get_imm12(0x010_0_0000), 0x010);
  assert_eq!(get_imm12(0x020_0_0000), 0x020);
  assert_eq!(get_imm12(0x040_0_0000), 0x040);
  assert_eq!(get_imm12(0x080_0_0000), 0x080);
  assert_eq!(get_imm12(0x100_0_0000), 0x100);
  assert_eq!(get_imm12(0x200_0_0000), 0x200);
  assert_eq!(get_imm12(0x400_0_0000), 0x400);

  //-- Pruebs de signo
  assert_eq!(get_imm12(0x800_0_0000), 0xFFFF_F800u32 as i32); //-- -2048
  assert_eq!(get_imm12(0xFFF_0_0000), 0xFFFF_FFFFu32 as i32); //-- -1
  assert_eq!(get_imm12(0xFFF_FFFFF), -1); 
  assert_eq!(get_imm12(0x800_FFFFF), -2048);
  assert_eq!(get_imm12(0x7FF_FFFFF), 2047);
  assert_eq!(get_imm12(0xFFE_FFFFF), -2);
}

#[test]
fn test_sign_ext() {
  //-- Test de la función sign_ext
  assert_eq!(sign_ext(0x000), 0);
  assert_eq!(sign_ext(0x001), 1);
  assert_eq!(sign_ext(0x7FF), 2047);  //-- 0x7FF
  assert_eq!(sign_ext(0x800), -2048); //-- 0x800
  assert_eq!(sign_ext(0xFFF), -1);
  assert_eq!(sign_ext(0x7FF_FFFF), -1);
}

#[test]
fn test_is_type_i() {
  //-- Test de la función is_type_i

  //-- Tipos correctos
  assert!(is_type_i(OPCODE_I_ARITH));
  assert!(is_type_i(OPCODE_I_LOAD));
  assert!(is_type_i(0x13));  //-- ADDI
  assert!(is_type_i(0x03));  //-- LW

  //-- Tipos incorrectos
  assert!(!is_type_i(0x00));
  assert!(!is_type_i(0x01));
  assert!(!is_type_i(0x02));
  assert!(!is_type_i(0x04));
  assert!(!is_type_i(0xFF));
}

#[test]
fn test_disassemble_addi() {
    //-- Test de la funcion disassemble
    //-- Instrucciones addi 

    assert_eq!(disassemble(0x00000013), "addi x0, x0, 0");
    assert_eq!(disassemble(0x00100093), "addi x1, x0, 1");
    assert_eq!(disassemble(0x00200113), "addi x2, x0, 2");
    assert_eq!(disassemble(0xfff00193), "addi x3, x0, -1");
    assert_eq!(disassemble(0x7ff00213), "addi x4, x0, 2047");
    assert_eq!(disassemble(0x00308f93), "addi x31, x1, 3");
    assert_eq!(disassemble(0x00410413), "addi x8, x2, 4");
    assert_eq!(disassemble(0x00820813), "addi x16, x4, 8");
    assert_eq!(disassemble(0x01040893), "addi x17, x8, 16");
    assert_eq!(disassemble(0xff040893), "addi x17, x8, -16");
    assert_eq!(disassemble(0x80040893), "addi x17, x8, -2048");
    assert_eq!(disassemble(0x0aa00093), "addi x1, x0, 170");    
}

#[test]
fn test_disassemble_slli() {
    //-- Test de la funcion disassemble
    //-- Instrucciones slli

    assert_eq!(disassemble(0x00111093), "slli x1, x2, 1");
    assert_eq!(disassemble(0x00001013), "slli x0, x0, 0");
    assert_eq!(disassemble(0x00209f93), "slli x31, x1, 2");
    assert_eq!(disassemble(0x00411f13), "slli x30, x2, 4");
    assert_eq!(disassemble(0x00819e93), "slli x29, x3, 8");
    assert_eq!(disassemble(0x01021e13), "slli x28, x4, 16");
    assert_eq!(disassemble(0x01129d93), "slli x27, x5, 17");
    assert_eq!(disassemble(0x01e31d13), "slli x26, x6, 30");
    assert_eq!(disassemble(0x01f39c93), "slli x25, x7, 31");
}

#[test]
fn test_disassemble_slti() {
    assert_eq!(disassemble(0x00112093), "slti x1, x2, 1");
    assert_eq!(disassemble(0x00002013), "slti x0, x0, 0");
    assert_eq!(disassemble(0x0020af93), "slti x31, x1, 2");
    assert_eq!(disassemble(0x00412f13), "slti x30, x2, 4");
    assert_eq!(disassemble(0x0081ae93), "slti x29, x3, 8");
    assert_eq!(disassemble(0x01022e13), "slti x28, x4, 16");
    assert_eq!(disassemble(0x0112ad93), "slti x27, x5, 17");
    assert_eq!(disassemble(0x01e32d13), "slti x26, x6, 30");  
    assert_eq!(disassemble(0x01f3ac93), "slti x25, x7, 31");  
}

#[test]
fn test_disassemble_sltiu()
{
    assert_eq!(disassemble(0x00113093), "sltiu x1, x2, 1");
    assert_eq!(disassemble(0x00003013), "sltiu x0, x0, 0");
    assert_eq!(disassemble(0x0020bf93), "sltiu x31, x1, 2");
    assert_eq!(disassemble(0x00413f13), "sltiu x30, x2, 4");
    assert_eq!(disassemble(0x0081be93), "sltiu x29, x3, 8");
    assert_eq!(disassemble(0x01023e13), "sltiu x28, x4, 16");
    assert_eq!(disassemble(0x0112bd93), "sltiu x27, x5, 17");
    assert_eq!(disassemble(0x01e33d13), "sltiu x26, x6, 30");
    assert_eq!(disassemble(0x01f3bc93), "sltiu x25, x7, 31");
}