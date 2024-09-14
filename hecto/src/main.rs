use std::io::{self, Read};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() {
    enable_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() {
                    println!("Binary: {0:08b} ASCII: {0:#03} \r", b);
                } else {
                    println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, c);
                }

                if c == 'q' {
                    break;
                }
            }
            Err(e) => {
                println!("Error: {e}");
                break;
            }
        }
    }

    disable_raw_mode().unwrap();
}
