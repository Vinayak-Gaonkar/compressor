use std::fs::File;
use std::io::{self, Read, Write};
use std::io::BufReader;

pub fn read_file(input_path: &str)-> std::io::Result<()>{
    let mut buf =Vec::new();
    let mut file = File::open(input_path)?;
    // let reader = BufReader::new(file);
    file.read_to_end(&mut buf);
    let compressed = rle_compressor(&buf);
    let mut output_file = File::create("./assets/output.rle")?;
    output_file.write_all(&compressed)?;
    
    Ok(())
}


fn rle_compressor(data: &Vec<u8>)-> Vec<u8> {

    let mut compressed = Vec::new();
    let mut i=0;

    while i<data.len(){
        let mut count=1;
        // println!("{:?}", data[i]);
        while i+1 < data.len() && data[i]==data[i+1]{
            count +=1;
            i=i+1;
        }
        compressed.push(data[i]);
        compressed.push(count);
        i+=1;
    }
    compressed
}