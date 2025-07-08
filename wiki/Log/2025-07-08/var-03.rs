//-- Tipos y tamaños de variables

fn main() {

  //-- Variables enteras
  let a = 10;  //-- Inferida
  let b: u8 = 20;  //-- Byte
  let c: u16 = 30; //-- Media palabra
  let d: u32 = 40; //-- Palabra
  let e: u64 = 50; //-- Doble palabra

  //-- Imprmir el valor de la variable 
  println!("Tamaño de a: {} bytes", std::mem::size_of_val(&a));
  println!("Tamaño de b: {} bytes", std::mem::size_of_val(&b));
  println!("Tamaño de c: {} bytes", std::mem::size_of_val(&c));
  println!("Tamaño de d: {} bytes", std::mem::size_of_val(&d));
  println!("Tamaño de e: {} bytes", std::mem::size_of_val(&e));
}

