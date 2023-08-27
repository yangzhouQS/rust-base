#[derive(Debug, PartialEq)]
// 方法和函数
pub struct Student {
    name: &'static str,
    score: i32,
}

impl Student {
    pub fn new(name: &'static str, score: i32) -> Self {
        Student { name, score }
    }

    pub fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }
    pub fn set_score(&mut self, score: i32) {
        self.score = score;
    }
    pub fn get_name(&self) -> &'static str {
        self.name
    }
    pub fn get_score(&self) -> i32 {
        self.score
    }
}

fn main3() {
    let mut student: Student = Student::new("李四", 96);
    println!("name = {}, score = {}", student.get_name(), student.get_score());
    student.set_score(66);
    println!("{:?}", student);
}

fn hello2() {
    println!("hello world");
}

fn main() {
    let fn_ptr: fn() = hello2;
    println!("{:p}", fn_ptr);

    let other_fn = hello2;
    // println!("{:p}", other_fn);
    fn_ptr();
    other_fn();
}
// 0x7ff7a6461210
// hello world
// hello world