fn main() {

    //-- Crear una cadena dinámica
    let cad1 = String::from("🟢¡Holi!!!🙂");

    //-- Nuevo ámbito
    {
        //-- Crear otra cadena dinamica
        let cad2 = String::from("🟢Cadena 2");

        println!("{}", cad2)
    } //-- Aquí cad2 desaparece, y el espacio se libera

    //-- Imprimir la otra cadena
    println!("{}", cad1);

} //-- Aquí cad1 desaparece, liberando el espacio



