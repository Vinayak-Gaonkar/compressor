mod  rle;


fn main() {
    let input_path = "./assets/hello.txt";  // Path to the file you want to compress
    let output_path = "./assets/photos.txt.gz";  // Path for the compressed file

    // Call the compressor function and handle any errors
    // if let Err(e) = compressor::compress_file(input_path, output_path) {
    //     eprintln!("Error compressing file: {}", e);
    // } else {
    //     println!("File compressed successfully.");
    // }
    rle::read_file(input_path);

}
