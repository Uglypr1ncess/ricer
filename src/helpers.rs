use std::io::{stdin, stdout, Write};

pub fn get_str_input(prompt: &str) -> Option<String> {
    let mut user_input = String::new();

    loop {
        user_input.clear();

        println!("{prompt}");
        let _ = stdout().flush();

        stdin()
            .read_line(&mut user_input)
            .expect("Did not enter string");

        let input = user_input.trim();

        if input == "y" || input == "n" {
            return Some(input.to_string());
        }

        println!("Please enter 'y' or 'n'.");
    }
}
