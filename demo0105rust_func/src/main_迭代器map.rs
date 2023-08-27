fn main() {
    let v = [1, 2, 3, 4, 5, 6];
    let v2: Vec<i32> = v.iter().map(|x| x + 1).collect();
    println!("v2 = {:?}", v2);
}
// v2 = [2, 3, 4, 5, 6, 7]
