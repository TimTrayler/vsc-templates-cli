
extern crate dirs;

use std::env;

mod commands;
mod values;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "new" => {
                commands::new::new();
            }

            "delete" => {
                commands::delete::delete();
            }

            "save" => {
                commands::save::save();
            }

            "list" => {
                commands::list::list();
            }

            _ => {
                println!("Unknown command '{}'.", args[1]);
                println!("{}", values::SYNTAX_MSG);
            }
        }
    } else {
        println!("No command specified.");
        println!("{}", values::SYNTAX_MSG);
    }
}
