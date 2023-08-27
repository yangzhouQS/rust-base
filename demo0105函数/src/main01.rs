fn add(a: i32, b: i32) -> i32 {
    return a + b; // 函数体
}

fn add2((a, b): (i32, i32)) -> i32 {
    return a + b;
}

fn add3(t: (i32, i32)) -> i32 {
    return t.0 + t.1;
}

fn main() {
    let a = 10;
    let b = 30;
    println!("add: {} + {} = {}", a, b, add(a, b));
    println!("add1: {} + {} = {}", a, b, add2((a, b)));
    println!("add2: {} + {} = {}", a, b, add3((a, b)));
    println!("Hello, world!");
}
