/*
1.变量区分为可变与不可变两种状态
2.变量遮蔽允许同名变量的覆盖
*/
fn main() {
    //1.变量声明
    let a = 100;
    println!("a = {}", a);

    // 2.变量的可变性
    let x = 100;
    // let声明的变量是只读的,禁止修改值
    // x = 200; // cannot assign twice to immutable variable `x`
    println!("x = {}", x);

    // 声明可变变量可以在变量前加上mut
    let mut y = 100;
    y = y + 99;
    println!("y = {}", y);

    // 3.变量遮蔽
    let a1 = 100;
    println!("a1 = {}", a1); // a1 = 100
    let a1 = 200;
    println!("a1 = {}", a1); // a1 = 200

    // 4. 常量,常量是绑定到标识符不允许改变的值
    // 常量必须指定数据类型,建议大写变量标识符
    const PI: f64 = 3.1415906;
    println!("PI = {}", PI)
}
