// 1,静态字符串
// 2,动态字符串
// 3,字符串的实现
// 4,创建字符串
// 5,连接字符串

use std::mem::*;

fn fn1() {
    // 字符串变量赋值是可以进行修改的
    let mut a = "hello";
    println!("a = {}", a);
    a = "DSB";
    println!("a = {}", a);


    // &str表示对str的引用,在标准库中将str定义为表示UTF-8字符串的不可修改字节数组的类型
    let a: &str = "";
    let b: &str = "0123456789";
    let c: &str = "hello";

    // 0 10 5
    println!("{} {} {}", size_of_val(a), size_of_val(b), size_of_val(c));

    // x64:16 16 16 变量本身大小
    println!("{} {} {}", size_of_val(&a), size_of_val(&b), size_of_val(&c));

    // x64:8 8 8  变量本身引用的大小
    println!("{} {} {}", size_of_val(&&a), size_of_val(&&b), size_of_val(&&c));

    // size_of_val()函数饭hi参数引用的对象的大小
    // &str类型不是仅包含一个指针的普通Rust引用,而是包含一对指针和一个长度组成的一对
}

// 动态字符串
fn fn2() {
    println!("---------------动态字符串---");
    let mut a: String = "H".to_string();
    a.push('e');
    a.push('l');
    a.push('l');
    a.push('o');
    println!("a = {}", a); // a = Hello

    // 在指定位置添加字符
    let mut b: String = "123".to_string();
    b.insert(1, 'A'); // 1A23
    b.remove(0);// 返回值为删除字符(根据位置索引删除) A23
    b.pop(); // 删除尾部字符A2
    b.push('1'); // A21
    b.push('4'); // A214
    println!("b = {}", b);
}

fn fn3() {
    println!("---------fn3--------");
    let a: &str = "";
    let b: &str = "0123456789";
    let c: &str = "abcdé";

    // 0 10 6
    println!("{} {} {}", size_of_val(a), size_of_val(b), size_of_val(c));

    // print打印变量本身大小,此类变量包含指针对象和usize对象,因此大小是普通引用大小的两倍.
    // 16 16 16
    println!("{} {} {}",
             size_of_val(&a),
             size_of_val(&b),
             size_of_val(&c)
    );


    // print语句打印变量本身的引用的大小
    // 8 8 8
    println!("{} {} {}",
             size_of_val(&&a),
             size_of_val(&&b),
             size_of_val(&&c)
    );
}

// 字符串的实现
fn fn4() {
    println!("---------fn4 字符串的实现 --------");
    let mut s1 = "".to_string();
    s1.push('e');

    let mut s2 = "".to_string();
    s2.push('é');

    let mut s3 = "".to_string();
    s3.push('€');
    println!("capacity: {}, len: {}", s1.capacity(), s1.len());
    println!("capacity: {}, len: {}", s2.capacity(), s2.len());
    println!("capacity: {}, len: {}", s3.capacity(), s3.len());
}

fn fn5() {
    println!("---------fn5 --------");
    let mut s1 = "".to_string();
    for _ in 0..10 {
        println!("{:?} {} {}",
                 s1.as_ptr(),
                 s1.capacity(),
                 s1.len()
        );
        s1.push('a');
    }
    println!("{:?} {} {}",
             s1.as_ptr(),
             s1.capacity(),
             s1.len()
    );
}


// 创建空字符串
fn fn6() {
    println!("---------fn6 创建空字符串 --------");

    // 构造函数方式创建
    let s1 = String::new();

    // 转换器构造函数
    let s2 = String::from("abc");
    let s3 = "abc".to_string();
    let s4 = "abc".to_owned();

    // 返回string对象
    let s5 = format!("abc");
    println!("{},{},{},{},{}", s1, s2, s3, s4, s5);

    // 静态字符串
    let s = "hello word";

    // 转换为动态字符串
    println!("{}", String::from(s));
    println!("{}", s.to_string());
    println!("{}", s.to_owned());
}

// 连接字符串
fn fn7() {
    println!("---------fn7--------");
    let s1 = "he";
    let s2 = "llo";
    let d1 = s1.to_string();
    let d2 = s2.to_string();

    // 1,静态字符串拼接
    let s3 = format!("{}{}", s1, s2);
    println!("s3 = {}", s3);

    // 2,静态字符串 + 动态字符串
    let s4 = format!("{}{}", s1, d2);
    println!("s4 = {}", s4);

    // 3,动态字符串 + 静态字符串
    let s5 = format!("{}{}", d1, s2);
    println!("s5 = {}", s5);

    // 4,动态字符串 + 动态字符串
    let s6 = format!("{}{}", d1, d2);
    println!("s6 = {}", s6);
}

fn fn8() {
    println!("---------fn8---------");
    let mut s1 = "Hello".to_string();
    s1 = format!("{}{}", s1, " word");
    s1 = format!("{}{}", s1, " 世界你好!");
    println!("s1 = {}", s1);
    // 使用format宏有点冗长

    let mut s2 = "hello".to_string();
    s2.push_str(" word");
    s2.push_str(" 世界你好!");
    println!("s2 = {}", s2);


    // push_str紧促形式
    let mut s3 = "Hello".to_string();
    s3 += " word";
    s3 += "!";
    // +=运算符应用与String对象是,等效于push_str函数
    s3.push_str(" 世界你好!");
    println!("s3 = {}", s3);

    // 要把动态字符串作为push_str 或 +=的参数传递, 必须事先将其转换为静态字符串. 使用&运算符可以获得这种效果
    let word = "bye".to_string();
    let w1: &str = &word;
    let w2: &String = &word;
    println!("{} {}", w1, w2);
}

fn main() {
    fn1();
    fn2();
    fn3();
    fn4();
    fn5();
    fn6();
    fn7();
    fn8();
    println!("Hello, world!");
}
