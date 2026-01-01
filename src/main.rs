mod cv_handler;
mod constants;
mod helpers;

pub use crate::cv_handler::CvHandler;
use crate::helpers::get_str_input;

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

    let user_input = match get_str_input(
        "Would you like to see all the possible programs you could add to the csv?"
    ) {
        Some(answer) => answer,
        None => return,
    };
    
    if user_input.trim() == "y" {
        for (program, values) in &program_values {
            println!("program: {program}, values: ");
            for value in values {
                println!("{value}")
            }
        }
    }
}
