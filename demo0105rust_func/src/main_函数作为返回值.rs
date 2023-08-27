// #[derive(Debug, PartialEq)]
type MathOp = fn(i32, i32) -> i32;

fn math(op: MathOp, x: i32, y: i32) -> i32 {
    println!("{:p}", op);
    op(x, y)
}

fn math_op(op: &str) -> MathOp {
    match op {
        "add" => add,
        _ => subtract
    }
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

fn main() {
    let (x, y) = (4, 5);
    println!("{} + {} = {}", x, y, math(add, x, y));
    println!("{} - {} = {}", x, y, math(subtract, x, y));
}
// 0x7ff722d61330
// 4 + 5 = 9
// 0x7ff722d61370
// 4 - 5 = -1