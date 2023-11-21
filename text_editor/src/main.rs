use std::io;

const X_SIZE:usize = 100;
const Y_SIZE:usize = 10;
const FILL_CHAR:char = '_';

struct Position (u32,u32);

fn main() {
    let mut cursor = Position(32,5);
    let mut screen = [[FILL_CHAR;X_SIZE];Y_SIZE];

    screen[cursor.1 as usize][cursor.0 as usize] = '%';

    print_screen(screen);
}

fn print_screen(screen:[[char;X_SIZE];Y_SIZE]) {
    for row in screen {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}
