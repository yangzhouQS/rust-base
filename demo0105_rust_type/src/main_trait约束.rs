use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy)]
struct Rectangle {
    width: f32,
    height: f32,
}

#[derive(Clone, Copy)]
struct Circle {
    radius: f32,
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

impl Geometry for Circle {
    fn area(&self) -> f32 {
        3.14 * self.radius * self.radius
    }

    fn perimeter(&self) -> f32 {
        2.0 * 3.14 * self.radius
    }
}

// 同时实现 Geometry trait和 Display trait
impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Rectangle:({}, {})", self.width, self.height)
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Circle:({})", self.radius)
    }
}

fn print(geometry: impl Geometry + Display) {
    println!("{} print: area = {}, perimeter = {}", geometry, geometry.area(), geometry.perimeter());
}

// impl trait 支持多个参数
fn area_add<T: Geometry, U: Geometry>(geo1: T, geo2: U) {
    println!("rect.area = {}, rect.perimeter = {} \ncircle.area = {}, circle.perimeter = {}",
             geo1.area(), geo1.perimeter(), geo2.area(), geo2.perimeter());
}

fn main() {
    let rect = Rectangle { width: 8.0, height: 6.0 };
    let circle = Circle { radius: 3.0 };
    print(rect);
    print(circle);
    area_add(rect, circle);
}

// Rectangle:(8, 6) print: area = 48, perimeter = 28
// Circle:(3) print: area = 28.26, perimeter = 18.84
// rect.area = 48, rect.perimeter = 28
// circle.area = 28.26, circle.perimeter = 18.84