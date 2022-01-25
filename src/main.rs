#[macro_use]
extern crate quick_error;

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

// Our goal is to have:
// 1) our own error type that deals with all the various errors we'll encounter
// 2) readable custom error messages, with context

fn main() {
    let result = do_something();
    println!("completed call to do_something()");
    match result {
        Ok(v) => println!("result tuple: {:?}", v),
        Err(e) => println!("do_something(): {}", e),
    }
}

fn do_something() -> Result<(String, u32), MyError> {
    let f = "example.txt";      // try "example2.txt" to force an error
    let num_as_string = "x2021"; // try "x2021" to force an error
    let s = read_file(f).map_err(|e| format!("I/O error while reading '{}': {}", f, e))?;
    let r = num_as_string
        .parse::<u32>()
        .map_err(|e| format!("error parsing '{}' as int: {}", num_as_string, e))?;
    Ok((s, r))
}

fn read_file(f: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(f)
}
