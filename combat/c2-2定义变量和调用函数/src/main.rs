fn main() {
    let a = 10; // 自动推断
    let b: i32 = 20; // 声明变量类型
    let c = 30i32; // 数字字面量加入类型
    let d = 30_i32; // 尾部注释类型
    let e = add(add(a, b), add(c, d));
    println!("(a + b) + (c + d) = {}", e);
}

// println! 是一个宏，与函数很类似，函数返回的是值，宏返回的是代码

fn add(i: i32, j: i32) -> i32 {
    return i + j;
}
