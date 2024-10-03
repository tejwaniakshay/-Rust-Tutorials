fn main() {
    println!("Hello, world!");
    let x = 10;
    println!("value of x is {x}");

    let mut y = x;
    y+=1;

    println!("value of y is {x}");
    println!("value of y is {y}");

    
    let a = "hello this is the string ";
    let b = a;
    println!("{b}");
    println!("{a}");
    let a = String::from("using the string data type");
    let b = a;
    //b = "new string".to_string();
    //println!("{a}");// --> cannot be used as this will cause double free memory problem
    println!("{b}")

}
