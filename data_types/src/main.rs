fn main() {
    println!("Data Types");


    //scalar types: data types where we can store single value
    // Integer : (8,26,32,64,128,arch)  -- signed(i)- i8  or unsigned(u) - u8,u16

    let no = 2;
    println!("{no}");

    let bol = true;
    println!("{bol}");
    //String, char

    let st = "this is the string";
    println!("{st}");
    //Floating

    let ft = 3.14;
    println!("{ft}");


    //Compoud types: data types where we can store multiple data types
    //Array, tuples, dictionary,set

    //tuples
    let tup = (34,23,15);
    println!("{:?}",tup);
    println!("{}", tup.1);

    let mut tup1: (i32, u16, f64) = (-32,34,45.67);
    println!("{:?}",tup1);

    tup1.0 = -100;
    println!("{:?}",tup1);

    //Arrays

    let a = [1,2,3,4,5];
    println!("{:?}",a);

    println!("{}",a[2])
}
