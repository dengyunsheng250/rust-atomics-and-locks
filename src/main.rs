fn main() {
    //f(a,b) 中的a,b不可能是对于一个元素的引用
}

fn f(a: &i32, b: &mut i32) {
    let before = *a;
    *b += 1;
    let after = *a;
    if before != after {
        x();
    }
}

fn x() {}
