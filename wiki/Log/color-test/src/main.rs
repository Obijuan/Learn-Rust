use text_colorizer::*;

fn main() {
    println!("{} - Prueba. Color verde", "Hello, world!".green());
    println!("{} - Prueba. rojo", "Rojo".red().bold());
    println!("{}", "Reversed".blue().reversed());
    println!("{}", "Italic".yellow().italic());
    println!("{}", "Strikethrought".strikethrough());
    println!("{}", "Clear".white().clear());
    println!("{}", "Dimmed".white().dimmed());
    println!("{}", "Underline".magenta().underline());
    println!("{}", "Blink".white().hidden());
}
