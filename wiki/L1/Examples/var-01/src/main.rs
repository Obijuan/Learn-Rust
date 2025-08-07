fn main() {

    //-- Ejemplo de variable de 32 bits
    let x: i32 = 10;
    let y: u8 = 255;
    let z: i32 = y as i32 + x as i32;
    let mut a: i32 = 0xff;
    let b: u8 = 0b11;

    println!("El valor de x es: {x}");
    println!("y: {y}");
    println!("{x} + {y} = {z}");
    println!("a: {a}");
    println!("b: {b}");
    a = 10;
    println!("a: {a}");
    println!("res: {}", 10_000 + 11_000);

    for i in 1..=10 {
        println!("i: {i}");
    }

    for elem in [1, 2, 3, 4, 5] {
        println!("Elemento: {elem}");
    }
}
