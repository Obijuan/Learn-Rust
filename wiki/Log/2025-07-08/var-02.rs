//-- Ejemplos de variables
//-- Cálculo del tamaño de una variable

fn main() {

  //-- Crear una variable entera
  let a = 10;

  //-- Imprmir el valor de la variable 
  println!("Tamaño de a: {} bytes", std::mem::size_of_val(&a));

}

