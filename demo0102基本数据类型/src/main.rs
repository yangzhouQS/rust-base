/*
基本数据类型
1. 整型
2. 浮点数
3. 布尔数据类型
4. 字符类型
5. 范围类型
*/
fn main() {
    // 1. 整数类型
    let inter1: u8 = 100; // 无符号8位
    let inter2: i8 = 100; // 有符号8位
    let _inter3: i16 = 1080; // 有符号16位
    let _inter4: i32 = 1200; // 有符号32位

    // help: if this is intentional, prefix it with an underscore: `_inter5`
    // let inter5: i64 = 1300; // 有符号64位 // 定义的变量未使用,建议变量名称前面使用下划线解决
    let _inter5: i64 = 1300; // 有符号64位
    let inter6: u128 = 1000000; // 有符号128位
    println!("inter1={}\ninter2={}", inter1, inter2);
    println!("inter6={}", inter6);

    let a1: u32 = 0b100001; // 二进制
    let a2: u32 = 0o21; // 八进制
    let a3: u32 = 0x111; // 十六进制
    println!("a1={} \na2={} \na3={}", a1, a2, a3);

    // 2. 浮点数,
    // f32:单精度浮点型
    // f64:双精度浮点型
    let f: f32 = 3.14;
    let f2: f64 = 3.151515151515;
    println!("f={}\nf2={}", f, f2);

    //3.布尔数据类型
    let b1: bool = false;
    let b2 = true;
    println!("b1={},b2={}", b1, b2);

    // 4.字符类型,每个字符占四个字节
    let z = 'z';
    let q: char = '1';
    println!("z = {},1={}", z, q);

    // 5.范围类型
    for index in 1..5 {
        println!("index = {}", index);
    }
    print!("包含5");
    for index in 1..=5 {
        println!("index = {}", index);
    }
    print!("逆序\n");
    for index in (1..=5).rev() {
        println!("index = {}", index);
    }
    print!("求和\n");
    let sum: u32 = (1..=5).sum();
    println!("sum = {}", sum);
}
