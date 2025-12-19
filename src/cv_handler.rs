use std::fs;
use std::collections::HashMap;

pub struct CvHandler {
    pub file_path: String
}

impl CvHandler {
    pub fn new(path: String) -> CvHandler {
        CvHandler { file_path: path }
    }

    pub fn read_cv(&self) {
        let contents = fs::read_to_string(&self.file_path)
            .expect("Can't read file path");
    
        let mut program_paths: HashMap<String, String> = HashMap::new();

        let mut program = "";
        let mut path = "";

        let mut buffer: String = String::new();
        let mut counter = 0;

        for letter in contents.chars() {
            if letter == ',' && counter == 0 {
                program = &buffer;
                counter = 1;
            } else if letter == ',' && counter == 1 {
                path = &buffer;
                program_paths.insert(String::from(program), String::from(path));
                counter = 0;
            } else {
                buffer.push(letter);
            }
        }

        for (program, path) in &program_paths {
            println!("program: {}, path: {}", program, path);
        }
    }
}
