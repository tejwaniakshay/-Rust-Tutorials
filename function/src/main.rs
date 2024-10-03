fn main() {
    
    first_fun();
    second_fun(12);

    let res = add(10,45);
    println!("returned result is equal to {}", res);
    expr();

}

fn first_fun(){
    println!("this is the first function");
}

//passing the data type as well
fn second_fun(x : i32){
    println!("the value of x is {x}");

}


fn add(a : i32, b : i32) -> i32{
    println!("the value of a is {a} and b is {b}");
    println!("result is equal to {}",a+b);
    a+b
}

fn expr(){
    let y = {
        let x = 10;
        x+1
    };
    println!("the value of y is {}", y);
}