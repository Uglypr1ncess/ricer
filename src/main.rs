mod cv_handler;
mod constants;

pub use crate::cv_handler::CvHandler;

use std::io::{stdin, stdout, Write};
use std::path::Path;

const PROGRAM_PATHS: &str = "/Users/uglyprincess/Documents/Code/Rust/macice/src/programs.csv";

fn main() {
    
    let program_values = constants::build_programs();

    let cv_handler = CvHandler::new(String::from(PROGRAM_PATHS));
    
    let program_paths = cv_handler.read_cv();
    
    println!("current programs: ");
    for (program, path) in program_paths {
        println!("{program}, {path}");
        let file_path = Path::new(&path);

        if !file_path.exists() {
            eprintln!("Error: Couldn't open {path}");
            return;
        }
    }
    
    let mut user_input = String::new();

    loop {
        user_input.clear();

        println!("Would you like to see all the possible programs you could add in the csv? (y/n)");
        let _ = stdout().flush();

        stdin().read_line(&mut user_input).expect("Did not enter string");

        let input = user_input.trim(); // removes \n
    
        if input == "y" || input == "n" {
            break;
        }

        println!("Please enter 'y' or 'n'.");
    }
    if user_input.trim() == "y" {
        for (program, values) in &program_values {
            println!("program: {program}, values: ");
            for value in values {
                println!("{value}")
            }
        }
    }
}
