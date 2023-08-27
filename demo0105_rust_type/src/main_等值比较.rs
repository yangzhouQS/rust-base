use std::fmt::{Display, Formatter, Result};

enum BookFormat {
    Paperback,
    Hardback,
    Ebook,
}

#[derive(Debug)]
struct Book {
    isbn: i32,
    format: BookFormat,
}

impl Display for Book {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.isbn)
    }
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn
    }

    // rust 会根据eq方法自动实现判断两个实例是否相等的ne方法
    // fn ne(&self, other: &Self) -> bool {
    //     todo!()
    // }
}

fn main() {
    let book1 = Book { isbn: 3, format: BookFormat::Paperback };
    let book2 = Book { isbn: 3, format: BookFormat::Ebook };
    let book3 = Book { isbn: 6, format: BookFormat::Paperback };


    // assert!(book1 == book2);
    // assert!(book1 != book3);
    println!("{:?}", book1);
}