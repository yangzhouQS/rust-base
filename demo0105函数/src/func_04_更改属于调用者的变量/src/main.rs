// 修改函数参数传入的外部变量
fn double_variable(mut a: [i32; 5]) -> [i32; 5] {
    for index in 0..10 {
        a[index] *= 10;
    }
    return a;
}

fn main2() {
    let mut arr = [1, 2, 3, 4, 5];
    double_variable(arr);

    println!("arr = {:?}", arr);
}

fn double_negatives(a: &mut [i32; 5]) {
    for index in 0..5 {
        (*a)[index] *= 10;
    }
}

fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    // double_negatives(&mut arr);
    // println!("{:?}", arr);
}
