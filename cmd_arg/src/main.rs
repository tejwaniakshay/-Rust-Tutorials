

//using the environement module from the standard module 
use std::env;

fn main() {
    println!("command line arguments");

    let input_args = env::args();
    for input in input_args{
        println!("{}",input);
    }

    let input_args: Vec<String> = env::args().collect();
    for input in input_args.iter(){
        println!("{}",input);
    }

}
