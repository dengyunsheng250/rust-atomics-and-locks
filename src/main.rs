use std::sync::Arc;
use std::thread;
fn main() {
    let v = Arc::new(vec![1, 2, 3, 4]);
    let v1 = v.clone();
    let v2 = v.clone();
    let t1 = thread::spawn(move || {
        println!("{}", v1.as_ref().len());
    });
    let t2 = thread::spawn(move || {
        for v in v2.as_ref() {
            println!("{:?}", v);
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();
    println!("the main thread {:?} stops", thread::current().id());
}
