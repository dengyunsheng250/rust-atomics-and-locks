use std::thread;
static mut X: [i32; 3] = [1, 2, 3];
fn main() {
    let t1 = thread::spawn(move || unsafe {
        X[0] = 5;
    });

    let t2 = thread::spawn(move || unsafe {
        X[1] = 20;
    });
    t1.join().unwrap();
    t2.join().unwrap();
    unsafe {
        println!("{:?}", X);
    }
}
