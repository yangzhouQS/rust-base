fn main() {
    let v = [1, 2, 3];
    let iter = v.iter();

    // sum是对迭代器元素执行求和操作
    let sum: i32 = iter.sum();
    println!("sum = {}", sum); // sum = 6
}

