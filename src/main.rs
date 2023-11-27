use std::thread;
fn main() {
    let t1 = thread::spawn(f);
    t1.join().unwrap();
    let t2 = thread::spawn(f);

    println!("Hello from the main thread. {:?}", thread::current().id());

    t2.join().unwrap();
}

fn f() {
    println!("Hello from another thread");

    let id = thread::current().id();

    println!("This is my thread id: {id:?}");
}
