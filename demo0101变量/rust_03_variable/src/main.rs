// 函数参数和函数参数类型，形参
fn add(i: i32, j: i32) -> i32 {
    // 函数返回值类型
    return i + j; // 函数返回值
}
fn main() {
    let a = 10; // 编译器自动推断类型
    let b: i32 = 20; // 创建变量时，程序员声明类型
    let c = 30i32; //数字类型可以在数字的字面量中加入类型注解
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));

    println!("(a+b) + (c+d) = {}", e);
}
