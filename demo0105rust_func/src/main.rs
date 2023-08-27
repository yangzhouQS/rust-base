fn main() {
    let v = [1, 2, 3];
    let v2 = [4, 5, 6];
    let result: Vec<i32> = v.iter().zip(v2.iter())
        .map(|(a, b)| a + b)
        .filter(|x| x % 3 == 0)
        .collect();
    println!("{:?}", result);
}
// [9]