use std::fs;
use std::collections::HashMap;

pub struct CvHandler {
    pub file_path: String
}

impl CvHandler {
    pub fn new(path: String) -> CvHandler {
        CvHandler { file_path: path }
    }

    pub fn read_cv(&self) -> HashMap<String, String> {
        let contents = fs::read_to_string(&self.file_path)
            .expect("Can't read file path");
    
        let mut program_paths: HashMap<String, String> = HashMap::new();

       for line in contents.lines(){
           let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 2 {
                program_paths.insert(parts[0].to_string(), parts[1].to_string());
            }
       }
       program_paths
    }
}
