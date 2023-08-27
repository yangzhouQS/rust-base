// 定义函数

// add:函数参数,函数参数类型为i32,返回值数据类型也是i32
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let x = 5;
    let y = {
        let x = 2;
        x + 1
    };

    // sum:返回值 = 函数调用实参,实参()
    let sum = add(x, y);
    println!("{} + {} = {}", x, y, sum);
    println!("Hello, world!");
}
