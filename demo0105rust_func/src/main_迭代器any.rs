fn main() {
    let v = [1, 2];
    let mut iter = v.iter();

    // 迭代器any,检查是否满足条件的元素
    let result = iter.any(|x| x % 2 == 0);
    let result2 = iter.any(|&x| x == 0);
    println!("result = {:?}", result); // result = true
    println!("result2 = {:?}", result2); // result2 = false
}
