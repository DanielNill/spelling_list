extern crate clap;

//use std::env;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Spelling List")
        .version("0.1.0")
        .author("Daniel Nill")
        .about("Keep track of words you can't spell")
        .arg(Arg::with_name("INPUT")
             .help("Add a word to your list")
             .required(true)
             .index(1))
        .get_matches();

    let word = matches.value_of("INPUT").unwrap();
    
    let file = match OpenOptions::new().write(true).create(true).append(true).open("spelling_list.txt") {
        Ok(file) => file,
        Err(..) => panic!("Error recording word"),
    };
    let mut writer = BufWriter::new(&file);
    writer.write(word.as_bytes()).expect("Error recording word");
    writer.write(b"\n").expect("Error recording word");
    println!("'{}' added to your spelling list", word); 
}
