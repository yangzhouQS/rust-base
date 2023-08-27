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

// 定义原型
struct Circle {
    radius: f32,
}

// 实现计算圆周长和面积的方法
impl Geometry for Circle {
    fn area(&self) -> f32 {
        3.14 * self.radius * self.radius
    }

    fn perimeter(&self) -> f32 {
        2.0 * 3.14 * self.radius
    }
}

fn main() {
    let rect = Rectangle { width: 8.0, height: 6.0 };
    println!("rect.area = {}, rect.perimeter = {}", rect.area(), rect.perimeter());

    let circle = Circle { radius: 3.0 };
    println!("circle.area = {}, circle.perimeter = {}", circle.area(), circle.perimeter());
}

// rect.area = 48, rect.perimeter = 28
// circle.area = 28.26, circle.perimeter = 18.84