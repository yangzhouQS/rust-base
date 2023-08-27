fn double() -> f64 {
    0.34
}

fn f1(x: i32) -> i32 {
    5.5;
    "abc";
    3 + 7
}

fn f2(x: i32) -> i32 {
    4.5;
    "abc";
    78 + 1
}

fn f3(x: i32) -> i32 {
    4.5;
    "abc";
    78
}

// 提前退出
fn f4(x: i32) -> i32 {
    if x < 0 {
        return 0;
    }
    return x * 100;
}

// 返回多个值,可以返回元组
fn f5() -> (i32, i32, i32) {
    return (1, 2, 3);
}

fn main() {
    let f = double();
    println!("f = {}", f);
}
