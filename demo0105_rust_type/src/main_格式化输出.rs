use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    // 1.格式化输出Debug与Display
    let origin = Point { x: 1, y: 2 };
    println!("{}", origin);
    println!("{:?}", origin);
    println!("{:#?}", origin);
}
// (1, 2)
// Point { x: 1, y: 2 }
// Point {
//     x: 1,
//     y: 2,
// }