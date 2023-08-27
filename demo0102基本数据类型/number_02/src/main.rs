fn main() {
    let three = 0b11; // 二进制是0b开头的
    let thirty = 0o36; // 八进制是0o开头的
    let three_hundred_and_twenty = 0x12c; // 十六进制是0x开头的,以16为基数

    println!("three is: {}", three);
    println!("base 10: {},{},{}", three, thirty, three_hundred_and_twenty);
    println!(
        "base 2: {:b},{:b},{:b}",
        three, thirty, three_hundred_and_twenty
    );
    println!(
        "base 8: {:o},{:o},{:o}",
        three, thirty, three_hundred_and_twenty
    );
    println!(
        "base 16: {:x},{:x},{:x}",
        three, thirty, three_hundred_and_twenty
    );
}
