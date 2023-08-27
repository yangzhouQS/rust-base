// &str与String类型转换
fn main() {
    let x = String::from("hello word");
    let y = x.as_str();
    println!("x = {}, y = {}", x, y);

    let x = "hello word";
    let y = x.to_string();
    println!("x = {}, y = {}", x, y);
}

// x = hello word, y = hello word
// x = hello word, y = hello word