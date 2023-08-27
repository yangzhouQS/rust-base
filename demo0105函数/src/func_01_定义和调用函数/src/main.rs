fn line(){
    println!("------########");
}

// `line` must be defined only once in the value namespace of this module
// 同一个作用域内函数无法重复声明,不存在覆盖遮蔽
// fn line(){
//     println!("ok");
// }
fn main() {
    line();

    // 快内部的函数可以遮蔽全局的函数
    fn line(){
        println!("ok");
    }
    line();

    // rust允许函数嵌套

    f2();

    // 函数定义
    fn f2(){
        println!("函数嵌套");
    }

    // 函数调用
    f2();
}
