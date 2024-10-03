fn main() {
    println!("Tuple struct ");
    
    struct user(u8,String, f32);
    
    let mut user1 = user(32, "Akshay got the remote job".to_string(), 4.2);
    println!("{},{},{}", user1.0,user1.1,user1.2);
    user1.1 = "Akshay tejwani".to_string();
    println!("{},{},{}", user1.0,user1.1,user1.2)
    
}
