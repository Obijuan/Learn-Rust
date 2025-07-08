//-- Ejemplo booleanos

fn main() {

    let bool1 = true;
    let bool2 = false;
    let bool3 = bool1 && bool2;
    let bool4 = bool1 || bool2;
    let bool5 = !bool1;
    let bool6 = bool1 == bool2;
    let bool7 = bool1 != bool2;

    let var1: u8 = 42;
    let var2: u8 = 42;
    let var3: u8 = 3;
    let bool8 = var1 == var2;
    let bool9 = var1 != var3;
    let bool10 = var1 < var2;
    let bool11 = var1 > var3;

    //-- Conversion de booleanos a enteros
    let var4: u8 = bool1 as u8;
    let var5: u8 = bool2 as u8;

    println!("bool1: {}", bool1);
    println!("bool2: {}", bool2);
    println!("bool3 (bool1 && bool2): {}", bool3);
    println!("bool4 (bool1 || bool2): {}", bool4);
    println!("bool5 (!bool1): {}", bool5);
    println!("bool6 (bool1 == bool2): {}", bool6);
    println!("bool7 (bool1 != bool2): {}", bool7);
    println!("bool8 (var1 == var2): {}", bool8);
    println!("bool9 (var1 != var3): {}", bool9);
    println!("bool10 (var1 < var2): {}", bool10);
    println!("bool11 (var1 > var3): {}", bool11);
    println!("var4 (bool1 as u8): {}", var4);
    println!("var5 (bool2 as u8): {}", var5);

}
