

struct Person{
    name: String,
    height: i32
}

impl Person {
    fn get_name(&self) -> String{
        return  self.name.clone()
    }
    fn get_height(&self) -> i32{
        return  self.height.clone()
    }
    
}

fn main() {

    println!("impl keyword");
    let person1 = Person{
        name: String::from("this is the value of the stirng"),
        height: 43
    };
    let x = person1.get_name();
    let x1 = person1.get_height();
    println!("{x},{x1}")    
}
