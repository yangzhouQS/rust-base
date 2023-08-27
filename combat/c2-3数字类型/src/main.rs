// 整数和浮点数
fn fn1() {
    let twenty = 20; // 类型推断
    let twenty_one: i32 = 21; // 显式的添加类型
    let twenty_two = 22i32; // 添加类型后缀
    let addition = twenty + twenty_one + twenty_two;

    println!("{}+{}+{} = {}", twenty, twenty_one, twenty_two, addition);

    let on_million: i64 = 1_000_000; // 下划线可以增加代码的可读性
    println!("{}", on_million);

    // 定义数字数组
    let forty_twos = [3.14, 3.333, 66_f32, 66f32];
    println!("{:02}", forty_twos[0]);
}
fn fn2() {
    //      二进制 0b
    let three = 0b11;
    //     八进制
    let thirty = 0o36;
    //     十六进制 0x前缀表示十六进制，以16为基数
    let three_hundred = 0x12c;

    println!("base 10:{} {} {}", three, thirty, three_hundred);
    println!("base 2:{:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8:{:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16:{:x} {:x} {:x}", three, thirty, three_hundred);
}

use ::std::convert::TryInto;
// rust数字的比较运算
// rust 的类型安全性保证，不允许不同类型的数据进行比较
fn fn3() {
    let a: i32 = 10;
    let b: f64 = 20.0;
    // expected `i32`, found `f64`
    // if a < b {

    // 将f64类型转换位32位，向下转换数据类型
    if a < b as i32 {
        println!(" a < b");
    }

    // 优雅的错误处理
    let b_ = b.try_into().unwrap();
    if a < b_ {
        println!(" a < b");
    }
}
fn main() {
    // fn1();
    // fn2();
    fn3();
}
