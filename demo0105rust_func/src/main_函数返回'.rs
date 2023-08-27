// #[derive(Debug, PartialEq)]
type MathOp = fn(i32, i32) -> i32;

fn math(op: MathOp, x: i32, y: i32) -> i32 {
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
    println!("{}", math_op("add")(x, y));
    println!("{}", math_op("xyz")(x, y));
}
