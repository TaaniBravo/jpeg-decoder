use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = match args.get(1) {
        Some(path) => path,
        None => {
            eprintln!("Problem with parsing the file path");
            return;
        }
    };

    println!("File path: {}", file_path);

    let mut file = match File::open(file_path.as_str()) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Problem opening the file: {:?}", error);
            return;
        }
    };

    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_) => println!("Read {} bytes", buffer.len()),
        Err(error) => eprintln!("Problem reading the file: {:?}", error),
    }

    let mut hex_string: String = String::new();
    for byte in buffer.iter() {
        hex_string.push_str(format!("{:02X}", byte).as_str());
    }

    // println!("Hex: {}", hex_string);
    // Do something with the buffer
}
