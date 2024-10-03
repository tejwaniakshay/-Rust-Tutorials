struct Rectangle{
    height: u32,
    weight: u32
}

trait Desc{
    fn print_val(&self);
    fn disp_val(&self);
}

impl Desc for Rectangle{
    fn print_val(&self){
        println!("the value of rectangles are {},{}",{self.height} , {self.weight} );
    }
    fn disp_val(&self){
        println!("the area of rectangles is {}",{self.height} * {self.weight} );
    }
    
}

fn callbothfun<T:Desc> (shape: T){
    shape.print_val();
    shape.disp_val();
}

fn main() {

    println!("Traits practice ");
    let rect1 = Rectangle{
        height: 10,
        weight: 20
    };
    // rect1.disp_val();
    // rect1.print_val();
    callbothfun(rect1);
}
