#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        
    }
}

// mod lib;

use hello_cargo::front_of_house::hosting;
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn main() {
    // let greeting_file_result = File::open("hello.txt");

    let x = read_username_from_file();
    let f = match x {
        Ok(f) => f,
        Err(err) => panic!("i love it, {}",err),
    };
    // let greeting_file = match greeting_file_result {
        // Ok(file) => file,
        // Err(error) => panic!("Problem opening the file: {:?}", error),
    // };
}