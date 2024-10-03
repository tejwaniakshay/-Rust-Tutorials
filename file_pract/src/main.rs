
use std::{fs::File, io::{Read, Write}};

fn main() {
    println!("File practice");
    
    
    // lets create a new file  and write the file

    /* 
    let mut my_file = File::create("output.txt").expect("caannot create a file ");

    my_file.write(b"this is the contenet in tthe file").expect("error in writing the file");
    my_file.write(b"\n this the new content");

    */

    let mut my_file = File::open("output.txt").expect("cannot open the file");

    let mut content = String::new();
    
    my_file.read_to_string(&mut content).expect("not able to read the contents of the file");

    println!("{}",content);


}
