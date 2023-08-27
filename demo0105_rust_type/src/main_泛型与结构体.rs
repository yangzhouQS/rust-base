struct Rectangle1<T> {
    width: T,
    height: T,
}

struct Rectangle2<T, U> {
    width: T,
    height: U,
}

impl<T> Rectangle1<T> {
    pub fn width(&self) -> &T {
        &self.width
    }
    pub fn height(&self) -> &T {
        &self.height
    }
}

impl Rectangle1<i32> {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

impl<T, U> Rectangle2<T, U> {
    pub fn width(&self) -> &T {
        &self.width
    }
    pub fn height(&self) -> &U {
        &self.height
    }
}

fn main() {
    let rect1 = Rectangle1 { width: 10, height: 20 };
    println!("width = {}, height = {}", rect1.width(), rect1.height());
    println!("react1 area = {}", rect1.area());

    let react2 = Rectangle2 { width: 10, height: 2.22 };
    println!("react2.width = {}, react2.height = {}", react2.width(), react2.height());
}

// width = 10, height = 20
// react1 area = 200
// react2.width = 10, react2.height = 2.22