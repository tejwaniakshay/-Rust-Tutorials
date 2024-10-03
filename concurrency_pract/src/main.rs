use std::thread::{self, sleep};
use std::sync::atomic::{self, AtomicI16};
use std::time::Duration;


static  counter:AtomicI16 = {
    let v = 0;
    AtomicI16::new(v)
};
fn atomicinc(x: i16){
    if counter.load(atomic::Ordering::SeqCst) > 100{
        return;
    }
    counter.fetch_add(1, atomic::Ordering::SeqCst);
    println!("counter = {}",counter.load(atomic::Ordering::SeqCst)+x);
    sleep(Duration::from_nanos(1));
}

fn main() {
    println!("Concurrency practice ");

    // let handler = thread::spawn(|| {

    //     for i in 1..10{
    //         println!("This is the value {}",i);
    //         sleep(Duration::from_millis(1));
    //     }
    // });

    let handler = thread::spawn(|| {
        for i in 1..16{
            atomicinc(100);
        }
    });
    let handler1 = thread::spawn(|| {
        for i in 1..16{
            atomicinc(0);
        }
    });
    
    let _ = handler1.join().unwrap();
    let _ = handler.join().unwrap();
    // for i in 1..10{
    //     println!("i = {}", i );
    // }

}
