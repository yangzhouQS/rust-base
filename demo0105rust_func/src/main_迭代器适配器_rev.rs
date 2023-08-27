fn main() {
    let v = [1, 2, 3, 4, 5, 6];
    let result = v.iter().rev();
    for value in result {
        println!("value = {}", value);
    }
}
// value = 6
// value = 5
// value = 4
// value = 3
// value = 2
// value = 1