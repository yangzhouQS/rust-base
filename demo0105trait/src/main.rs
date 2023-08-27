mod mod1 {
    pub const HELLO: &str = "hello world";
    /*
    trait Shape {
    fn area(&self) -> i64;
    }*/
}

fn main() {
    assert_eq!(32, 32);
    println!("{}", mod1::HELLO);
    println!("Hello, world!");
}
