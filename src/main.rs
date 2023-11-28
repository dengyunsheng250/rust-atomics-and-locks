use std::sync::Arc;
use std::thread;
fn main() {
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();

    let t1 = thread::spawn(|| dbg!(a));
    let t2 = thread::spawn(|| dbg!(b));
    t1.join().unwrap();
    t2.join().unwrap();
}
