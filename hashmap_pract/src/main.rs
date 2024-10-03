use std::collections::HashMap;


fn main() {
    println!("Hash map practice");

    let mut marks: HashMap<&str, i8> = HashMap::new();
    marks.insert("maths1", 32);
    marks.insert("maths2", 12);
    marks.insert("maths3", 2);
    marks.insert("maths4", 22);
    marks.insert("maths5", 52);

    for mark in marks.iter(){
        println!("subject: {} = {}", mark.0,mark.1);
    }
}
