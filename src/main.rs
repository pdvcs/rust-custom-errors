#[macro_use]
extern crate quick_error;

use std::fs;
use std::error::Error;

quick_error! {
    #[derive(Debug)]
    pub enum MyError {
        IO(err: std::io::Error) {
            display("I/O error: {}", err)
            from()
        }
        Parse(err: std::num::ParseIntError) {
            display("Integer parse error: {}", err.description())
            from()
        }
    }
}

fn main() {
    let x = do_something();
    println!("completed call to do_something()");
    match x {
        Ok(v) => println!("function returned tuple: {:?}", v),
        Err(e) => println!("function returned error: {}", e),
    }
}

fn do_something() -> Result<(String, u32), MyError> {
    let f = "example.txt";
    let s = read_file(f)?; // std::io::Error converted to MyError
    let r = "2018".parse::<u32>()?; // std::num::ParseIntError converted to MyError
    Ok((s, r))
}

fn read_file(f: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(f)
}
