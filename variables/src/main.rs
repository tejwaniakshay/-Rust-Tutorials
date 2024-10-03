fn main() {
    println!("Different types of data types");
    
    // immutable variable
    let a = 45; 
    //a= a+1;
    println!("the value of a is {}",a);
    println!("the value of a is {a}");
    
    //mutuable variable 
    let mut a1 = 45; 
    a1= a1+1;
    println!("the value of a is {}",a1);
    println!("the value of a is {a1}");


    let mut st = "this is the rust string ";
    println!("{st}");
    st = "now the value has been changed ";
    println!("{st}")

    
}
