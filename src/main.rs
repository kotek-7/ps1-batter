use core::panic;
use std::env;
use std::io::Write;
use std::path;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let ps1_path = path::PathBuf::from(args[1].clone());

    if !(ps1_path.exists() && ps1_path.is_file()) {
        panic!("Incorrect argument! Pass a correct path of a file for the argument.");
    }
    if ps1_path.extension().unwrap() != "ps1" {
        panic!("Incorrect argument! Pass a path of a .ps1 file for the argument.");
    }

    let bat_path = ps1_path.with_extension("bat");

    let mut bat_file = match fs::File::create(&bat_path) {
        Ok(file) => file,
        Err(err) => panic!("{}", err),
    };
    let bat_content = generate_bat_content(&ps1_path);
    match bat_file.write_all(bat_content.as_bytes()) {
        Ok(_) => println!("Successfully wrote to {}", bat_path.to_str().unwrap()),
        Err(err) => panic!("{}", err),
    }
}

fn generate_bat_content(ps1_path: &path::Path) -> String {
    let ps1_path_string = ps1_path.to_str().unwrap();
    format!("@echo off\r\npowershell -NoProfile -ExecutionPolicy Unrestricted {ps1_path_string}")
}
