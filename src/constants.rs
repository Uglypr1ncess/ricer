use std::collections::HashMap;

pub fn build_programs() -> HashMap<String, Vec<String>>{
    let program_values: HashMap<String, Vec<String>> = HashMap::from([
        (
            "sketchybar".to_string(),
                vec![
                    "8392".to_string(),
                    "jasiodf".to_string(),
                ],
        ),
        (
            "spicetify".to_string(),
                vec![
                    "8392".to_string(),
                    "jasiodf".to_string(),
                ],
        ),
        (
            "alacritty".to_string(),
                vec![
                    "8392".to_string(),
                    "jasiodf".to_string(),
                ],
        ),
        (
            "discord".to_string(),
                vec![
                    "8392".to_string(),
                    "jasiodf".to_string(),
                ],
        ),
        ( 
            "vscode".to_string(),
                vec![
                    "8392".to_string(),
                    "jasiodf".to_string(),
                ],
        ),
        (
            "nvim".to_string(),
                vec![
                    "8392".to_string(),
                    "jasiodf".to_string(),
                ],
        
        ),
    ]);

    program_values
}
