fn main() {
    println!("understanding LOOPS");

    f1();
    f2();
    f3();
}


// types of loop

//loop
//while
//for

fn f1(){
    let mut x = 0;
    loop{
        x+=1;
        println!("x = {x}");
        if x == 4 {
            println!("x = 4");
            break
        }

    }
}


fn f2(){
    let mut no = 0;
    while no < 5{
        println!("{no}");
        no+=1;
    }

    let mut i = 0;
    let a = [10,20,30,40,50];
    while i < 5 {
        println!("{}",a[i]);
        i+=1;
    }
}

fn f3(){

    for x in 1..7{
        println!("{x}");
    }
}