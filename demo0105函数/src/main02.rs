#[derive(Debug, PartialEq)]
pub struct Student {
    name: String,
    score: i32,
}

impl Student {
    pub fn new(name: String, score: i32) -> Self {
        Student { name, score }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn score(&self) -> i32 {
        self.score
    }
}

fn main() {
    let s1: Student = Student {
        name: String::from("liBai"),
        score: 86,
    };
    println!("name: {}, score: {}", s1.name(), s1.score());
}