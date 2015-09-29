extern crate glob;
extern crate rustc_serialize;
extern crate rustache;

use std::error::Error;
use std::process;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use glob::glob;
use rustc_serialize::json::Json;
use std::env;

fn load(path: &Path) -> String {
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
    return s;
}

fn read(path: &Path, tmpl: &String) {
    let s = load(path);
    let data = Json::from_str(&s).unwrap();
    let rv = rustache::render_text(tmpl, data).unwrap().unwrap();
    let output = String::from_utf8(rv).unwrap();
    println!("{}", output);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} template dataPath", args[0]);
        process::exit(1);
    }
    let tmpl = load(Path::new(&args[1]));
    let mut data_path = args[2].clone();
    if !data_path.ends_with("/") {
        data_path.push('/');
    }
    data_path.push_str("**/*.json");
    println!("{}", data_path);

    for entry in glob(&data_path).unwrap() {
        match entry {
            Ok(path) => read(&path, &tmpl),
            Err(e) => println!("{:?}", e)
        }
    }
}
