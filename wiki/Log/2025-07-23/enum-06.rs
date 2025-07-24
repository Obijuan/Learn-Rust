//-- Pruebas de tipos enumerados

//-- Tipo enumerado para representar un registro RISCV
//-- Hay que implementar la interfaz Clone y Copy para
//-- pasar el valor por copia a la funcion as_str()
#[derive(Clone, Copy)] 
enum Reg {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    X31,
}    

impl Reg {
    //-- Convertir el registro a cadena
    fn as_str(&self) -> String {
        format!("x{}", *self as u8)
    }
}

fn main() {
    //-- Imprimir el valor de un registro
    let r= Reg::X10;
    println!("Registro {}", r.as_str());
}   

