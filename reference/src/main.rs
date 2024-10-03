fn main() {
    println!("Reference variable ");

    let mut x = 10;
    let _y = &mut x;
    *_y += 1;
    println!("the value of x is {x}");
}
