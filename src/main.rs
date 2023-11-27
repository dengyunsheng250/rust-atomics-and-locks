use std::thread;
fn main() {
    let v = vec![1, 2, 3];

    let _ = thread::spawn(move || {
        for num in v {
            println!("{}", num);
        }
    })
    .join()
    .unwrap();
}
