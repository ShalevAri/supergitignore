use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use std::fs::{self, File};
use std::io::{self, prelude::*};

fn main() {
    let gitignore_path = ".gitignore";
    let gitignore_template_path = "src/gitignore.txt";

    let gitignore_content = match fs::read_to_string(gitignore_template_path) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Error reading gitignore template file. Make sure '{}' exists.", gitignore_template_path);
            return;
        }
    };

    if fs::metadata(gitignore_path).is_ok() {
        let options = ["Add to existing .gitignore", "Replace existing .gitignore", "Abort"];

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("A .gitignore file already exists. Choose an option:")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => {
                // Add to existing .gitignore
                let mut file = File::open(gitignore_path).expect("Unable to open .gitignore file");
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Unable to read .gitignore file");

                contents.push_str("\n");
                contents.push_str(&gitignore_content);

                fs::write(gitignore_path, contents).expect("Unable to write to .gitignore file");

                println!("Added to existing .gitignore file.");
            }
            1 => {
                // Replace existing .gitignore
                fs::write(gitignore_path, gitignore_content).expect("Unable to write to .gitignore file");
                println!("Replaced existing .gitignore file.");
            }
            2 => {
                // Abort
                println!("Operation aborted. No changes made.");
            }
            _ => unreachable!(),
        }
    } else {
        // .gitignore does not exist, create a new one
        fs::write(gitignore_path, gitignore_content).expect("Unable to create .gitignore file");
        println!("Created a new .gitignore file.");
    }
}
