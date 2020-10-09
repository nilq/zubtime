extern crate rustyline;
extern crate colored;

#[macro_use]
#[cfg(feature = "serde_derive")] 
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use std::env;
use colored::*;

mod zubtime;
use zubtime::io::*;

fn nice(again: bool) {
    let new_project = again || env::args().len() == 1;

    let mut rl = Editor::<()>::new();

    println!("Welcome to {}", "Zub-time©".red().on_yellow());

    let mut zub = if new_project {
        println!();
        println!("It's new projecc time.");

        let name = rl.readline("Pick a name>>").unwrap();
        let author = rl.readline("And who you are>>").unwrap();
        let path = rl.readline("Aaand a path>>").unwrap();

        println!("\n{}\n", "Let's go.".green().bold());

        ZubFile::new(
            name,
            path,
            author
        )
    } else {
        match ZubFile::from(&env::args().collect::<Vec<String>>()[1]) {
            Some(zub) => zub,
            None => {
                println!("{}\n", "Not so good. No file here.".red());
                return nice(true)
            }
        }
    };

    loop {
        let readline = rl.readline(">>");

        match readline {
            Ok(line) => {
                println!("{}", line)
            },
            Err(ReadlineError::Interrupted) => {
                println!("Saving ...");

                zub.save();

                break
            },
            Err(ReadlineError::Eof) => {
                println!("Goodbye. Everything is saved.");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}

fn main() {
    nice(false)
}