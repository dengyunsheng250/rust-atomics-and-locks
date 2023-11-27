use std::thread;
fn main() {
    let v = vec![1, 2, 3, 4];

    thread::scope(|s| {
        s.spawn(|| {
            println!(
                "{:?} computes the sum of v is {}",
                thread::current().id(),
                v.iter().sum::<i32>()
            );
        });
        s.spawn(|| {
            println!(
                "{:?} computes the product of v is {}",
                thread::current().id(),
                v.iter().product::<i32>()
            )
        });
    });
    println!("the main thread {:?} stops", thread::current().id());
}
