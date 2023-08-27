// 数字与String类型转换
fn main() {
    let x = 7;
    let y = x.to_string();
    println!("i32: {}, String: {}", x, y);

    let x = 7.7;
    let y = x.to_string();
    println!("f64: {}, String: {}", x, y);

    let x = String::from("7");
    let y = x.parse::<i32>().unwrap();
    println!("String: {}, i32: {}", x, y);


    let x = String::from("7.7");
    let y = x.parse::<f64>().unwrap();
    println!("String: {}, f64: {}", x, y);
}
// i32: 7, String: 7
// f64: 7.7, String: 7.7
// String: 7, i32: 7
// String: 7.7, f64: 7.7