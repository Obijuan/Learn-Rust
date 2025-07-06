use std::{thread, time};


fn main() {

    //-- Borrar terminal
    cls();

    //-- Cursor off
    cursor_off();
   
    //-- Marco de la pantalla
    draw_box(1,1,23,80);

    let mut x = 2;
    let y = 2;

    let mut xold = x;
    let mut yold = y;

    locate(x,y);
    println!("☻");

    let delay = time::Duration::from_millis(50);

    for i in 2..79 {
        locate(xold, yold);
        print!(" ");
        x = i;
        locate(x,y);
        println!("☻");
        xold = x;
        yold = y;

        thread::sleep(delay);


    }

    //-- Poner el cursor al final
    locate(1,24);

    //-- Cursor on
    cursor_on();
}

//----------------------------------------------
//-- cls: Borrar el terminal
//-- y situar el cursor en la posición 1,1
//----------------------------------------------
fn cls() {
    //-- Borrar el terminal
    print!("{esc}[2J", esc = 27 as char);

    //-- Situar cursor en posicion 1,1
    locate(1,1);

}

//-----------------------------------------
//-- Locate(x,y)
//-- Poner el curso en la posicion x,y
//-- x entre 1 - 80
//-- y entre 1 - 24
//-----------------------------------------
fn locate(x : u8, y : u8) {
  print!("{esc}[{y};{x}H", esc = 27 as char);
}

//-- Draw horizontal simple line
// fn draw_hsline(x : u8, y:u8, w:u8) {

//   for i in x .. x + w {
//     locate(i, y);
//     print!("─");
//   }

// }

fn draw_box(x:u8, y:u8, h:u8, w:u8) {

    //-- Primera línea
    locate(x,y);
    print!("┌");

    for i in x+1 .. x + w - 1 {
        locate(i, y);
        print!("─");
    }

    print!("┐");

    //-- Lineas intermedias
    for i in y+1 .. y + h - 1 {
        locate(x, i);
        print!("│");
        locate(x + w - 1, i);
        print!("│");
    }

    //-- Ultima línea
    locate(x, y+h-1);
    print!("└");
    for i in x+1 .. x + w - 1 {
        locate(i, y+h-1);
        print!("─");
    }
    
    print!("┘");
}

fn cursor_on() {
    println!("{esc}[?25h", esc = 27 as char);
}

fn cursor_off() {
     //-- Cursor off
     println!("{esc}[?25l", esc = 27 as char);
}