/*
复合数据类型
1. 元组
2. 数组
3. 结构体
4. 枚举
*/

fn main() {
    // 1. 元组,元组类型是由一个或者多个类型组合而成的类型
    let tup1 = (1, 2, "a", false);

    // 通过 元组名.索引数字 方式输出元组中的各个元素
    println!("tup1 = {:?}", tup1); // (1, 2, "a", false)
    println!("tup1.0 = {}", tup1.0); //  tup1.0 = 1

    // 2. 数组类型
    // 2.1 声明和初始化数组
    // a>指定数组数据类型和初始化长度进行初始化
    let arr: [i64; 5] = [1, 2, 3, 4, 56];
    println!("arr = {:?}", arr);

    // b>省略数组类型进行初始化
    let arr2 = ["a", "b", "c"];
    println!("arr2 = {:?}", arr2);

    // c>默认值初始化
    let arr3: [i32; 5] = [8; 5];
    println!("arr3 = {:?}", arr3); // arr3 = [8, 8, 8, 8, 8]

    // 数组长度
    println!("arr3.len() = {}", arr3.len()); // arr3.len() = 5

    // 数组遍历
    for index in 0..arr3.len() {
        println!("arr3[{}] = {}", index, arr3[index]);
    }

    // 迭代数组 iter()
    for value in arr3.iter() {
        println!("value = {}", value);
    }

    // 可变数组,默认情况下let创建的数组不可以改变,具体表现为:a>数组变量不可以重新赋值;b>数组元素不可以修改
    // let users = ["lisi", "大牛", "李白", "白居易"];
    // users[1] = "王之涣"; // help: consider changing this to be mutable: `mut users`
    let mut users = ["lisi", "大牛", "李白", "白居易"];
    users[1] = "王之涣";
    println!("users = {:?}", users); // ["lisi", "王之涣", "李白", "白居易"]

    // 3. 结构体
    struct Employee { // 定义结构体
        name: String,
        company: String,
        age: u32,
    }
    // a>创建结构体实例
    let lisi = Employee {
        name: "李四".to_string(),
        company: "科长".to_string(),
        age: 26,
    };
    println!("lisi name= {},company = {},age = {}", lisi.name, lisi.company, lisi.age);\
    // 结构体实例默认是 不可修改的，因为结构体实例也是一个使用 let 定义的变量。
    // 如果要修改结构体实例，就必须在创建时添加 mut 关键字，让它变成可修改的。

    // 4. 枚举
    println!("Hello, world!");
}
