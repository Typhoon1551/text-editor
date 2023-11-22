//dependencies
use std::io;

//constants
const X_SIZE:usize = 100;
const Y_SIZE:usize = 10;
const FILL_CHAR:char = ' ';

//structs

fn main() {
    let mut cursor: u32 = 3;
    let mut screen = [[FILL_CHAR;X_SIZE];Y_SIZE];

    //loop {
        print_screen(&mut screen,&cursor);

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();
    //}
}

fn print_screen(screen:&mut [[char;X_SIZE];Y_SIZE], cursor:&u32) {
    println!("{}{}{}","+---+","-".repeat(X_SIZE),'+');

    let mut line:u32 = 0;

    for row in screen {
        print!("|{: >3}|",if line == cursor.to_owned() {"%%%".to_string()} else {line.to_string()});
        for col in row {
            print!("{}", col);
        }
        println!("|");
        line += 1;
    }

    println!("{}{}{}","+---+","-".repeat(X_SIZE),'+');
}
