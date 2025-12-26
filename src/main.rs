mod cv_handler;
mod constants;

pub use crate::cv_handler::CvHandler;

use std::collections::HashMap;

const PROGRAM_PATHS: &str = "/Users/uglyprincess/Documents/Code/Rust/macice/src/programs.csv";

fn main() {
    
    let program_values = constants::build_programs();



    let cv_handler = CvHandler::new(String::from(PROGRAM_PATHS));
    
    let program_paths = cv_handler.read_cv();
    
    println!("current programs: ");
    for (program, path) in program_paths {
        println!("{program}, {path}");                          
    }
    println!("if you'd wish for more programs, you can add them in programs.csv. Current supported programs: ") 

    

}
