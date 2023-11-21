//dependencies
use std::io;

//constants
const X_SIZE:usize = 100;
const Y_SIZE:usize = 10;
const FILL_CHAR:char = ' ';

//structs

fn main() {
    let mut cursor: (u32, u32) = (0,0);
    let mut screen = [[FILL_CHAR;X_SIZE];Y_SIZE];

    loop {
        print_screen(&mut screen,&cursor);

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() == "quit" {
            break;
        }

        let input = input.trim().chars().next().unwrap();

        screen[cursor.1 as usize][cursor.0 as usize] = input;

        if cursor.0 < X_SIZE as u32 - 1 {
            cursor.0 += 1;
        } else {
            cursor.0 = 0;
            cursor.1 += 1;
        }
    }
}

fn print_screen(screen:&mut [[char;X_SIZE];Y_SIZE], cur:&(u32,u32)) {
    screen[cur.1 as usize][cur.0 as usize] = '%';
    
    println!("{}{}{}","+---+","-".repeat(X_SIZE),'+');

    let mut line:u32 = 1;

    for row in screen {
        print!("|{: >3}|",line);
        for col in row {
            print!("{}", col);
        }
        println!("|");
        line += 1;
    }

    println!("{}{}{}","+---+","-".repeat(X_SIZE),'+');
}
