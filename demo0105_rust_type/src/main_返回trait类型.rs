use std::fmt::{Display, Formatter, Result};

struct Rectangle {
    width: f32,
    height: f32,
}

// trait可以包含两种形式的方法:抽象方法(没有具体实现的方法)和具体实现的方法(带有具体实现的方法)
trait Geometry {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
}

// 实现计算矩形周长和面积的计算方法
impl Geometry for Rectangle {
    // 面积
    fn area(&self) -> f32 {
        self.width * self.height
    }

    // 周长
    fn perimeter(&self) -> f32 {
        2.0 * (self.width + self.height)
    }
}


// 同时实现 Geometry trait和 Display trait
impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Rectangle:({}, {})", self.width, self.height)
    }
}

fn print(geometry: impl Geometry + Display) {
    println!("{} print: area = {}, perimeter = {}", geometry, geometry.area(), geometry.perimeter());
}

fn return_geometry() -> impl Geometry {
    Rectangle {
        width: 3.0,
        height: 4.0,
    }
}

fn main() {
    let rect = return_geometry();
}
