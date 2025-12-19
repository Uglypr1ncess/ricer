mod cv_handler;

pub use crate::cv_handler::CvHandler;

const PROGRAM_PATHS: &str = "/Users/uglyprincess/Documents/Code/Rust/macice/src/programs.csv";

fn main() {
    let cv_handler = CvHandler::new(String::from(PROGRAM_PATHS));
    cv_handler.read_cv();
}
