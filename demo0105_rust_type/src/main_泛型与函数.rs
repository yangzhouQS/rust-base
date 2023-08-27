fn foo<T>(x: T) -> T {
    return x;
}

fn main() {
    println!("foot = {}", foo(100.23));
    println!("foot = {}", foo("hello"));
    println!("Hello, world!");
}
// foot = 100.23
// foot = hello
