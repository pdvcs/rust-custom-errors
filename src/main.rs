#[macro_use]
extern crate quick_error;

use std::error::Error;
use std::fs;

quick_error! {
    #[derive(Debug)]
    pub enum MyError {
        Custom(s: String) {
            display("{}", s)
            from()
        }
    }
}

// Here our goal is to have readable custom error messages, with context

fn main() {
    let result = do_something();
    println!("completed call to do_something()");
    match result {
        Ok(v) => println!("result tuple: {:?}", v),
        Err(e) => println!("do_something(): {}", e),
    }
}

fn do_something() -> Result<(String, u32), MyError> {
    let f = "example.txt";
    let num_as_string = "2018";
    let s = read_file(f).map_err(|e| format!("I/O error while reading '{}': {}", f, e))?;
    let r = num_as_string.parse::<u32>().map_err(|e| {
        format!("error parsing '{}' as int: {}", num_as_string, e.description())
    })?;
    Ok((s, r))
}

fn read_file(f: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(f)
}