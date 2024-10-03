#[derive(Debug)] //definging for the programmer to view the output, agar compliter ko pta nahi h ki output kaise dekhega 
enum Color{
    Red(i32),
    Yellow,
    Green
}

/*
fn match_Exp(col: Color) -> i32{
    match col{
        Color::Red => {
            println!("this isi the red color");
            1
        }

        Color::Yellow => {
            println!("this isi the Yellow color");
            2
        }

        Color::Green => {
            println!("this isi the green color");
            3
        }
    }
}
 */

fn main() {
    
    println!("Hello, world!");
    let x = Color::Red(43);
    println!("{:?}",x);

    let y = cal(01);
    println!("{:?}", y )
}

//option enums

fn cal(no: i32) -> Option<bool> {
    if no == 0{
        None
    }
    
    else{
        Some(false)
    }
}

