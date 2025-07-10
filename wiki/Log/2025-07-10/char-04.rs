//-- Probando caracteres unicode

fn main() {

    let chars = [
        'a', // Letra a
        'ñ', // Letra ñ
        'ü', // Letra ü
        '😊', // Emoji
        '🟢', // Círculo verde
        '🚀', // Cohete
        '─'  // Línea horizontal
    ];

    //-- Imprimir los caracteres
    for c in chars.iter() {
        print!("{} ", c);
    }
    println!();
    
}
