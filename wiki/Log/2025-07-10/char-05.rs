
fn main() {

   //-- Ejemplo de caracter ascii
   let char1: u8 = b'a';

   //-- Tamaño de char1
   println!("Tamaño de char1: {} byte", std::mem::size_of_val(&char1));

   //
   println!("Valor de char1: {}", char1);
   println!("Impresión como carácter: {}", char1 as char);
}
