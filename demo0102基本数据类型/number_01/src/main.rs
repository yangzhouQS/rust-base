fn main() {
    let twenty = 20; // 编译器自己推断类型
    let twenty_two: i32 = 21; // 显示指定类型
    let twenty_three = 22i32; // 添加类型注解后缀

    let addition = twenty + twenty_two + twenty_three;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_two, twenty_three, addition
    );

    let one_million: i64 = 1_000_000; // 下划线增加了程序的可读性

    // 执行数字本身提供的一些方法
    println!("One million is {}", one_million.pow(2));

    // 创建一个数字数组
    let forty_twos = [42.0, 42f32, 42.0_f32];

    // 数组元素使用数字作为索引进行访问
    println!("{:02}", forty_twos[0]);
}
