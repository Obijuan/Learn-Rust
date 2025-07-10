//-- Funcion que devuelve una cadena
//-- La cadena devuelta debe ser estática
//-- Es una referencia que nunca se libera
//-- y por tanto no desaparece al acabar la función
fn saludo_formal(formal: bool) -> &'static str {
    if formal {
        "Hola, ¿cómo está usted?"
    } else {
        "🟢¡Holi!!!🙂"
    }
}


fn main() {

    println!("{}", saludo_formal(true));    

}

