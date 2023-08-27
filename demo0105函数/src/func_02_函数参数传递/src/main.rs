// 函数参数需要指定参数的类型
fn print_sum(num: f64, num2: f64) {
    println!("{} + {} = {}", num, num2, num2 + num);
}

fn main2() {

    // 实参要和函数参数需要的类型一致
    print_sum(10.0, 3.1415);
}


fn print_double(mut x: f64) {
    x *= 10.;
    println!("print_double x = {}", x);
}

fn main() {
    let x = 3.14;

    // 传递给函数的不是变量,而是变量的值
    print_double(x);
    println!("main x = {}", x);
}
