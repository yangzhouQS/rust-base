fn main() {
    let v = [1, 2, 3];
    let mut iter = v.iter();
    println!("{:?}",iter.next());
    println!("{:?}",iter.next());
    println!("{:?}",iter.next());
    println!("{:?}",iter.next());
}

// Some(1)
// Some(2)
// Some(3)
// None