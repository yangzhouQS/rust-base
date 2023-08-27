use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};


#[derive(Eq)]
struct Person {
    id: i32,
    name: String,
    height: u32,
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {}, {})", self.id, self.name, self.height)
    }
}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        self.height.cmp(&other.height)
    }
}

impl PartialEq<Self> for Person {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
    }
}

impl PartialOrd<Self> for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let person1 = Person { id: 1, name: String::from("zhang_san"), height: 160 };
    let person2 = Person { id: 2, name: String::from("li_si"), height: 166 };
    let person3 = Person { id: 3, name: String::from("tom"), height: 175 };
    assert_eq!(person1 < person2, true);
    assert_eq!(person2 > person3, false);

    assert!(person1.lt(&person2));
    assert!(person3.lt(&person2));
}