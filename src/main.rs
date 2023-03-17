use std::{process::exit, io::Write};

use log::{info, warn};

fn main() {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let input = std::io::stdin();

        let mut buf = String::new();
        let res = input.read_line(&mut buf);
        match res {
            Ok(n) => info!("read message size {}", n),
            Err(e) => warn!("{}", e),
        }

        match buf.trim() {
            "foo" => {
                println!("{}",buf);
            }
            "exit" => {
                println!("{}",buf);
                exit(0);
            }
            _ => {
                println!("do nothing {}",buf);
            }
        }
    }
}
