fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    /*--> src\main.rs:4:12
      |
      4 |     if a < b {
      |        -   ^ expected `i32`, found `u16`
      |        |
          |        expected because this is `i32`
      |
      help: you can convert a `u16` to an `i32`
      |
          4 |     if a < b.into() {
          |             +++++++
    */
    // 不同类型的数字禁止比较
    /*if a < b {
        println!("a is less than b");
    }*/

    // 从更小的类型转换到更大的类型是安全的，这种类型转换叫做类型提示
    if a < b as i32 {
        println!("a is less than b");
    }

    // try_into() 方法返回一个 Result包装过的i32类型
    let b_ = b.try_into().unwrap();

    if a < b_ {
        println!("a is less than b")
    }
}
