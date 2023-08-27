fn main() {
    println!("Hello, world!");
    let str = "grid body";
    let japanese = "日本語";
    let regions = [str, japanese]; // 数组字面量
    for region in regions.iter() {
        // iter返回迭代器
        println!("{}", &region);
    }
}
