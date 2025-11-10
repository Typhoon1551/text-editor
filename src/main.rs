//dependencies
use std::io;

//constants
const SIZE:usize = 10;
const CURSOR_CHAR:char = '%';

//structs

fn main() {
    let mut cursor: u8 = 0;
    let mut screen = vec![String::new();256];

    loop {
        print_screen(&mut screen,&cursor);

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();

        if input.chars().nth(0).unwrap() == '\\' {
            if input.chars().nth(1).unwrap() == '\\'{
                match input.trim() {
                    "\\\\quit" => break,
                    &_ => println!("Unknown command"),
                }
            } else {
                cursor = (input as &str)[1..].parse::<u8>().expect("invalid line number");
            }
        } else {
            screen[cursor as usize] = input.to_string();
            cursor += 1;
        }
    }
}

fn print_screen(screen:&mut Vec<String>, cursor:&u8) {
    println!("{}{}{}","+---+","-".repeat(100),'+');

    let mut number:u8 = 0;
    let screen_top = (cursor - (SIZE as u8/2)).clamp(0, 255) as usize;

    for line in &screen[screen_top..screen_top+SIZE] {
        println!("|{: >3}|{: <100}|",if number == cursor.to_owned() {CURSOR_CHAR.to_string().repeat(3)} else {number.to_string()}, line);
        number += 1;
    }

    println!("{}{}{}","+---+","-".repeat(100),'+');
    print!("{}:",number)
}
