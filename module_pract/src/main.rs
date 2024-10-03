mod my_module;
use my_module::my_function;

use my_module::person1;
mod moduel1{

    // by default function in modules are private, to access them we need to make them public using "pub" keyword
    pub fn person1(){
        println!("this is the first person function");
    }
}

fn main() {
    
    println!("Module practice ");
    moduel1::person1();
    my_function();
    person1();
    
}
