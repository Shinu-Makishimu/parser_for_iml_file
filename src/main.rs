use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;
use regex::Regex;

fn main(){
    parse_rust();
    fill_dirs();
}

fn parse_rust() {
    // Path to the rust.iml file
    let file_path = "rust.iml";

    // Open the file and create a buffered reader
    let file = File::open(file_path);
    let reader = io::BufReader::new(file);

    // Define $MODULE_DIR$ as "rust"
    let module_dir = "rust";

    // Regex to extract the path from the sourceFolder tag
    let re = Regex::new(r#"url="file://\$MODULE_DIR\$(.+?)""#).unwrap();

    // Read lines from the file
    for line in reader.lines() {
        let line = line?;

        // Check if the line contains a sourceFolder entry and extract the path
        if let Some(captures) = re.captures(&line) {
            let relative_path = &captures[1];

            // Combine $MODULE_DIR$ with the relative path
            let full_path = format!("{}/{}", module_dir, relative_path);

            // Create directories if they don't exist
            create_directories(&full_path);
        }
    }

    Ok(())
}

fn create_directories(path: &str) {
    let path_obj = Path::new(path);
    if !path_obj.exists() {
        // Create the necessary directories
        match fs::create_dir_all(path_obj) {
            Ok(_) => println!("Directories created: {}", path),
            Err(e) => println!("Failed to create directories: {}", e),
        }
    } else {
        println!("Path already exists: {}", path);
    }
}

fn fill_dirs() {
    let commons_dir = "rust/commons"; // Path to commons directory

    // Iterate over all subdirectories in commons
    if let Ok(entries) = fs::read_dir(commons_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                // Check if it's a directory and contains a src subdirectory
                if path.is_dir() && path.join("src").exists() {
                    let subdir_name = path.file_name().unwrap().to_str().unwrap();
                    create_cargo_toml(&path, subdir_name);
                    create_main_rs(&path.join("src"), subdir_name);
                }
            }
        }
    } else {
        println!("Failed to read the commons directory.");
    }
}

fn create_cargo_toml(dir: &Path, subdir_name: &str) {
    let cargo_toml_path = dir.join("Cargo.toml");
    let cargo_content = format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[dependencies]
"#,
        name = subdir_name
    );

    // Write Cargo.toml file
    match File::create(&cargo_toml_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(cargo_content.as_bytes()) {
                println!("Failed to write Cargo.toml for {}: {}", subdir_name, e);
            } else {
                println!("Created Cargo.toml for {}", subdir_name);
            }
        }
        Err(e) => println!("Failed to create Cargo.toml for {}: {}", subdir_name, e),
    }
}

fn create_main_rs(src_dir: &Path, subdir_name: &str) {
    let main_rs_path = src_dir.join("main.rs");
    let main_content = format!(
        r#"fn main() {{
    println!("This is {name}");
}}
"#,
        name = subdir_name
    );

    // Write main.rs file
    match File::create(&main_rs_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(main_content.as_bytes()) {
                println!("Failed to write main.rs for {}: {}", subdir_name, e);
            } else {
                println!("Created main.rs for {}", subdir_name);
            }
        }
        Err(e) => println!("Failed to create main.rs for {}: {}", subdir_name, e),
    }
}
