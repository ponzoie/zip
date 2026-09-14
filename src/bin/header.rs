use std::fs::File;
use std::io::{Result,prelude::*};

fn main() {
    let f = read_to_binfile();
}

fn read_to_binfile() {
    let filename = "../../test.txt";
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}",contents);
    Ok(())
}

fn write_to_binfile() -> Result<()>{
}