use std::env;
use std::fs::{File};
use std::io::Read;
use std::path::Path;

fn main() {
    let mut args = env::args();
    match args.len() {
        2 => {
            let file_path = args.nth(1).unwrap();
            let path_object = Path::new(file_path.as_str());

            if !path_object.exists() {
                println!("Input file does not exist, try creating it or inputting an existing file");
                return;
            }
            let mut file = File::open(&path_object).expect("File could not be opened");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Was not able to read from the input file, check if it's being used by another program or if it's available.");

            println!("{}", contents)
        },
        _ => {
            println!("Unexpected input, expected {}, got {}.", 1, args.len()-1);
            // Decrease the amount expect by 1 to account for the program name.
        }
    }
}
