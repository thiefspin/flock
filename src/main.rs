use std::env;
use std::fs;
#[macro_use]
extern crate magic_crypt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use magic_crypt::MagicCryptTrait;

fn main() {
    let params: Vec<String> = env::args().collect();
    let filename = &params[1];
    let new_file_path = Path::new("test.txt");
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mc = new_magic_crypt!("magickey", 256);

    let base64 = mc.encrypt_str_to_base64(contents);

    println!("With text:\n{}", base64);

    let mut file = match File::create(&new_file_path) {
        Err(why) => panic!("couldn't create {}: {}", new_file_path.display(), why),
        Ok(file) => file,
    };

    match file.write_all(base64.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", new_file_path.display(), why),
        Ok(_) => println!("successfully wrote to {}", new_file_path.display()),
    }
}
