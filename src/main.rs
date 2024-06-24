use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).unwrap();

    println!("input: {buffer:?}");

    println!("next sec;");

    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;

        if c == 'q' {
            println!("closing program.");
            break;
        }
    }
}

