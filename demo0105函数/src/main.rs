fn hello() {
    println!("hello 函数指针");
}

fn main22() {
    // 函数指针
    let fn_ptr: fn() = hello;
    let other_fn = hello; // the trait `Pointer` is not implemented for `fn() {hello}`
    // println!("{:p}", other_fn);

    // println!("{:p}", fn_ptr); // 0x7ff788951230
    fn_ptr();
    other_fn();
}

// 函数作为参数
fn sum(a: i64, b: i64, callback: fn(i64)) {
    callback(a + b);
}

fn result_sum(result: i64) {
    println!("result = {}", result);
}

fn main() {
    sum(10, 20, result_sum);
}