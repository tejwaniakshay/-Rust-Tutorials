fn main() {

    f1();
    f2();
    f3();
}

fn f1(){
    let no = 4 ;
    if no < 4 {
        println!("the number is less than 4")
    }
    else{
        println!("the number is greater than equal to 4");
    }
}

fn f2(){
    let no = 4;
    if no == 3 {
        println!("the number is equal to 3");
    }
    else if no == 4{
        println!("the number is equal to 4");
    }
    else{
        println!{"the number is not equal to either 3 or either 4"}
    }
}

fn f3(){
    let condition = true;
    let value = if condition == true {3} else {4};
    println!("{}",value);
}