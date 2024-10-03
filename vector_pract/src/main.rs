use std::ptr::null;

fn main() {
    println!("Vector ");
    let mut vec1 = Vec::<i32>::new();
    vec1.resize(10, 0);
    println!("{:?}",vec1);

    let mut multi_vec1 = Vec::<Vec<i32>>::new();
    for _ in 1..5{
        let mut temp_vec = Vec::<i32>::new();
        temp_vec.resize(10, 10);
        multi_vec1.resize(5, temp_vec);

    }

    println!("{:?}", multi_vec1 );

    println!("the lenght of vector is {}", multi_vec1.len());
    multi_vec1[0][0]= 50;
    println!("{:?}", multi_vec1 );


}
