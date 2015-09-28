extern crate glob;
extern crate toml;
use toml::Parser;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use glob::glob;

fn read(path: PathBuf) {
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => ()
    }
    let value = Parser::new(&s).parse().unwrap();
    println!("{} content:\n{:?}", display, value);
}

fn main() {
    for entry in glob("**/*.toml").unwrap() {
        match entry {
            Ok(path) => read(path),
            Err(e) => println!("{:?}", e)
        }
    }

    // for argument in env::args() {
    //     println!("{}", argument);
    // }
}
