mod node_core;

use chrono::Local;
use clap::Parser;
use node_core::commands::LsCommand;
use std::{io::Write, process::exit};

fn main() {
    println!("weclome pob, today is {}", Local::now());
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let input = std::io::stdin();

        let mut buf = String::new();
        input.read_line(&mut buf).unwrap();
        buf = buf.trim().to_owned();

        if buf.is_empty() {
            continue;
        }

        if buf == "exit" || buf == "quit" {
            println!("~bye~");
            exit(0);
        }

        let mut args = buf.split_whitespace();
        let command_name = args.next();
        match command_name {
            Some("ls") => {
                let cli = LsCommand::parse();
                println!("{:?}", cli);
            }
            Some(_) => {
                println!("other str")
            }
            None => println!("nothing to do"),
        }
    }
}
