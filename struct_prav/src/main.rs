
struct Employee{
    name: String,
    age: i32,
    email: String
}
fn main() {
    println!("Structure");

    
    let mut u1 = Employee{
        name: String::from("akshay"),
        age: 32,
        email: String::from("a@gmail.com")
    };

    u1.age = 28;
    println!("my name is {} and email id is {} and age is {}",u1.name,u1.email,u1.age);
    

    let u2 = Employee{
	name: "this is the new string".to_string(),
	age: 23,
	email: "asdlfj".to_string()
    };
    println!("this is the new string {}", u2.name);
}
