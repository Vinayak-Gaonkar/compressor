use std::fs::File;
use std::io::{self, Read, Write};

pub fn read_file(input_path: &str)-> std::io::Result<()>{
    let file = File::open(input_path)?;
    
    Ok(())
}
