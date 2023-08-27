fn main() {
    let v = [1, 2, 3, 4, 5, 6];
    let result: Vec<i32> = v.iter()
        .map(|x| x * 2)
        .filter(|&x| x % 2 == 0)
        .collect();
    println!("result: {:?}", result);
}
// result: [2, 4, 6, 8, 10, 12]