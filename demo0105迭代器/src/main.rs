/*fn main() {
    let v = [1, 2, 3];
    let mut iter = v.iter();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
}*/

fn main33() {
    let v = [1, 2, 3, 4];
    let total: i32 = v.iter().sum(); // total: 10
    let ret1: bool = v.iter().any(|&x| x > 4); // 数组元素是否有大于4的值
    let ret2: bool = v.iter().any(|&x| x > 2); // 数组元素是否有大于2的值
    let v2: Vec<i32> = v.iter().map(|x| x - 1).collect();
    println!("total: {}", total);
    println!("ret1 = {}, ret2 = {}", ret1, ret2); // ret1 = false, ret2 = true
    println!("{:?}", v2); // [0, 1, 2, 3]
}

fn main() {
    let v = [1, 2, 3, 4, 4, 5, 6];
    // map对数组元素每一项进行操作
    let ret1: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("ret1 = {:?}", ret1);

    // take返回迭代器中前n个元素
    let ret2 = v.iter().take(3);
    println!("ret2 = {:?}", ret2);
    // ret2 = Take { iter: Iter([1, 2, 3, 4, 4, 5, 6]), n: 3 }

    // filter对迭代器中每个元素调用闭包生成一个过滤元素的新迭代器
    let ret3: Vec<i32> = v.iter()
        .map(|x| x + 3)
        .filter(|&x| x % 3 == 0)
        .collect();
    println!("ret3 = {:?}", ret3);

    // rev:反转
    let ret4 = v.iter().rev();
    println!("ret4 = {:?}", ret4);

    // zip将两个迭代器压缩为一个迭代器
    let v1 = [1, 2, 3];
    let v2 = [2, 3, 6];
    let ret5: Vec<i32> = v1.iter().zip(v2.iter())
        .map(|(a, b)| a + b)
        .filter(|x| x % 3 == 0)
        .collect();
    println!("ret5 = {:?}", ret5);
}