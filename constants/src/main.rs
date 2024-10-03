fn main() {
    println!("Constants ");
    const const_value: i32 = 45;
    const str1: &str = " this is the string";
    // The below will not work because the String::from is not a constant
    //const str2: String =  "this is the new string ".to_string().as_str();
    println!("{}", str1)
}
