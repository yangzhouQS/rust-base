fn main() {
    let x: u16 = 7;
    let y = x as u32;
    println!("u32==> x: {}, y : {}", x, y);

    let x = std::u32::MAX;
    let y = x as u16;
    println!("u16==> x: {}, y : {}", x, y);

    let x = 65u8;
    let y = x as char;
    println!("char==> x: {}, y : {}", x, y);// x: 65, y : A

    let x = 'A';
    let y = x as u16;
    println!("u16==> x: {}, y : {}", x, y);

    let x = 7;
    let y = x as f64;
    println!("f64==> x: {}, y : {}", x, y);

    let x = 7.7;
    let y = x as i32;
    println!("i32==> x: {}, y : {}", x, y);
}

// u32==> x: 7, y : 7
// u16==> x: 4294967295, y : 65535
// char==> x: 65, y : A
// u16==> x: A, y : 65
// f64==> x: 7, y : 7
// i32==> x: 7.7, y : 7