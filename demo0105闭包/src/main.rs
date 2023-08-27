// rust中闭包就是匿名函数
fn main() {
    let say_hello = |str: String| {
        println!("sayHello {}", str)
    };
    say_hello(String::from("世界"));

    // 闭包捕获当前环境中的变量
    let num = 5;
    let closure = |a| println!("a={}, num={}", a, num);
    closure(10);
}
