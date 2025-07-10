//-- Ejemplos de cadenas

fn main() {

    //-- El tipo basico de las cadenas es &str
    //-- (Referencia a una cadena)
    //-- Las cadenas se define en utf8, y pueden contener
    //-- cualquier caracter unicode

    let cad1: &str = "🟢Hola!!🙂";
    let cad2: &str = "Holi!";

    //-- Imprimir las cadenas
    println!("Valor de cad1: {}", cad1);
    println!("Valor de cad2: {}", cad2);

}

